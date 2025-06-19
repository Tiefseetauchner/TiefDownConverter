# Lua Filters {#lua-filters}

Lua filters allow you to modify the document structure during Pandoc's
conversion step. They are attached to templates through the `filters`
field. The value may be a single Lua file or a directory containing
multiple filter scripts.

Pandoc executes the filters in the order they are listed. Filters can rename
headers, insert custom blocks or otherwise transform the document before it
reaches the template engine.

Example filter to adjust chapter headings:

```lua
function Header(el)
  if el.level == 1 then
    return pandoc.RawBlock("latex", "\\chapter{" .. pandoc.utils.stringify(el.content) .. "}")
  end
end
```

For more details on writing filters see the Pandoc documentation.
