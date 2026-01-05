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

local M = {}

local stringify = pandoc.utils.stringify

-- Convert a MetaMap representing a node into a plain Lua table.
local function normalize_node(m)
  return {
    id = m.id and m.id.value and stringify(m.id.value) or nil,
    path = m.path and stringify(m.path) or nil,
    title = m.title and stringify(m.title) or nil,
    prev = m.prev and m.prev.value and stringify(m.prev.value) or nil,
    next = m.next and m.next.value and stringify(m.next.value) or nil
  }
end

-- Build lookup tables from metadata
function M.build(meta)
  if not meta.nodes then
    return nil
  end

  local nodes = {}
  for _, m in ipairs(meta.nodes) do
    local n = normalize_node(m)
    if n.id then
      nodes[n.id] = n
    end
  end

  local current = nil
  if meta.current then
    current = normalize_node(meta.current)
  end

  return {
    nodes = nodes,
    current = current
  }
end

-- Look up a node by ID
function M.get_node(nav, id)
  if not nav or not nav.nodes then
    return nil
  end
  return nav.nodes[id]
end

return M
