local navlib = require 'navlib'

local DOCMETA = {}
local NAV = {}

local function build_nav(meta)
  local nav = navlib.build(meta)
  if nav then
    return nav
  end
  return {}
end

local function capture_meta(meta)
  DOCMETA = meta
  NAV = build_nav(meta)
  return meta
end

local function trim(s)
  return s:match('^%s*(.-)%s*$')
end

local function hex_to_char(x)
  return string.char(tonumber(x, 16))
end

local function unescape(url)
  return url:gsub('%%(%x%x)', hex_to_char)
end

local function dump(o)
  if type(o) == 'table' then
    local s = '{ '
    for k,v in pairs(o) do
        if type(k) ~= 'number' then k = '"'..k..'"' end
        s = s .. '['..k..'] = ' .. dump(v) .. ','
    end
    return s .. '} '
  else
    return tostring(o)
  end
end

local function resolve_path(path, root)
  local cur = root

  for segment in path:gmatch('[^%.]+') do
    local key, idx = segment:match('^([%w_]+)%[(%d+)%]$')
    if key then
      cur = cur and cur[key]
      if not cur then return nil end
      local i = tonumber(idx) + 1
      cur = cur[i]
    else
      cur = cur and cur[segment]
    end
    if not cur then return nil end
  end

  return cur
end

local function eval_meta_expr(expr)
  expr = trim(expr)
  if expr == '' then
    return ''
  end

  local value = resolve_path(expr, DOCMETA)
  if not value then
    return '{{' .. expr .. '}}'
  end

  return pandoc.utils.stringify(value)
end

local function create_lua_env()
  local env = {
    meta      = DOCMETA,
    M         = DOCMETA,
    pandoc    = pandoc,
    stringify = pandoc.utils.stringify,
    math      = math,
    string    = string,
    table     = table,
    dump      = dump,
    navlib    = navlib,
    nav       = NAV,
  }

  setmetatable(env, { __index = _G })
  return env
end

local function run_lua(expr)
  expr = trim(expr)
  if expr == '' then
    return nil, 'empty expression'
  end

  local env = create_lua_env()
  local chunk, err
  if expr:find('return', 1, true) then
    chunk, err = load(expr, 'meta-expr', 't', env)
  else
    chunk, err = load('return ' .. expr, 'meta-expr', 't', env)
  end

  if not chunk then
    return nil, err
  end

  local ok, result = pcall(chunk)
  if not ok then
    return nil, result
  end

  return result, nil
end

local function eval_lua_expr(expr)
  local value, err = run_lua(expr)

  if err then
    return '{{ lua: ' .. trim(expr) .. ' RETURNED ' .. err .. '}}'
  end

  if not value then
    return 'nil'
  end

  return pandoc.utils.stringify(value)
end

local function eval_condition(expr)
  local value, err = run_lua(expr)
  if value == nil or value == false then
    return false
  end
  return true
end

local function eval_expr(expr)
  expr = trim(expr)
  if expr == '' then
    return ''
  end

  if expr:lower():sub(1, 4) == 'lua:' then
    return eval_lua_expr(expr:sub(5))
  end

  return eval_meta_expr(expr)
end

local function resolve_expr(expr)
  return eval_expr(expr)
end

local function process_inlines(inlines)
  local result = pandoc.List()
  local i = 1

  while i <= #inlines do
    local el = inlines[i]

    if el.t == 'Str' then
      local text = el.text

      local replaced, count = text:gsub('{{(.-)}}', function(inner)
        return resolve_expr(inner)
      end)

      if count > 0 then
        result:insert(pandoc.Str(replaced))
        i = i + 1

      elseif text == '{{' then
        local j = i + 1
        local closing

        while j <= #inlines do
          local e = inlines[j]
          if e.t == 'Str' and e.text == '}}' then
            closing = j
            break
          end
          j = j + 1
        end

        if not closing then
          result:insert(el)
          i = i + 1
        else
          local slice = pandoc.List()
          for k = i + 1, closing - 1 do
            slice:insert(inlines[k])
          end

          local expr_text = pandoc.utils.stringify(pandoc.Inlines(slice))
          local expr_trim = trim(expr_text)
          local lower = expr_trim:lower()

          if lower:sub(1, 3) == 'if:' then
            local cond_expr = trim(expr_trim:sub(4))

            local fi_open, fi_close
            local p = closing + 1

            while p <= #inlines do
              local e = inlines[p]

              if e.t == 'Str' and e.text == '{{' then
                local q = p + 1
                local q_close

                while q <= #inlines do
                  local e2 = inlines[q]
                  if e2.t == 'Str' and e2.text == '}}' then
                    q_close = q
                    break
                  end
                  q = q + 1
                end

                if not q_close then
                  break
                end

                local inner_slice = pandoc.List()
                for r = p + 1, q_close - 1 do
                  inner_slice:insert(inlines[r])
                end

                local inner_expr = trim(pandoc.utils.stringify(pandoc.Inlines(inner_slice)))

                if inner_expr:lower() == 'fi' then
                  fi_open = p
                  fi_close = q_close
                  break
                end

                p = q_close + 1
              else
                p = p + 1
              end
            end

            if not fi_open then
              local replacement = resolve_expr(expr_text)
              result:insert(pandoc.Str(replacement))
              i = closing + 1
            else
              local cond_ok = eval_condition(cond_expr)

              if cond_ok then
                local inner = pandoc.List()
                for r = closing + 1, fi_open - 1 do
                  inner:insert(inlines[r])
                end

                local processed_inner = process_inlines(inner)
                for _, v in ipairs(processed_inner) do
                  result:insert(v)
                end
              end

              i = fi_close + 1
            end

          else
            local replacement = resolve_expr(expr_text)
            result:insert(pandoc.Str(replacement))
            i = closing + 1
          end
        end

      else
        result:insert(el)
        i = i + 1
      end

    elseif el.t == 'Link' then
      el.content = process_inlines(el.content)
      local target = unescape(el.target)
      if target:match('{{') then
        el.target = target:gsub('{{(.-)}}', function(inner)
          return resolve_expr(inner)
        end)
      end
      result:insert(el)
      i = i + 1

    else
      if el.content
         and type(el.content) == 'table'
         and el.content[1]
         and type(el.content[1]) == 'table'
         and el.content[1].t
      then
        el.content = process_inlines(el.content)
      end
      result:insert(el)
      i = i + 1
    end
  end

  return result
end

local function handle_block_inlines(el)
  el.content = process_inlines(el.content)
  return el
end

return {
  { Meta = capture_meta },
  {
    Para   = handle_block_inlines,
    Plain  = handle_block_inlines,
    Header = handle_block_inlines,
  },
}
