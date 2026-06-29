-- Custom pandoc writer that emits a document's metadata as plain JSON.
--
-- tiefdownconverter uses this to extract per-page front matter without having
-- to parse YAML/markdown itself: `pandoc <file> -t meta_writer.lua` prints the
-- front matter as a JSON object (or "{}" when there is none).

local function convert(value)
  local t = pandoc.utils.type(value)

  if t == "Inlines" or t == "Blocks" then
    return pandoc.utils.stringify(value)
  elseif t == "List" then
    local arr = {}
    for i, item in ipairs(value) do
      arr[i] = convert(item)
    end
    return arr
  elseif t == "table" or t == "Meta" then
    local obj = {}
    for key, item in pairs(value) do
      obj[tostring(key)] = convert(item)
    end
    return obj
  elseif t == "boolean" or t == "number" or t == "string" then
    return value
  else
    return pandoc.utils.stringify(value)
  end
end

function Writer(doc, opts)
  local meta = convert(doc.meta)
  -- Force object encoding so empty metadata serialises as "{}" rather than "[]".
  return pandoc.json.encode(setmetatable(meta, nil))
end
