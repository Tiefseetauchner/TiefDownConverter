# Lua filters

> Note: for documentation on Lua filters in Pandoc, refer to [the pandoc documentation](https://pandoc.org/lua-filters.html)

Lua filter integration in TiefDown is an important part of us manipulating documents. For example, to inject metadata, one can feasibly write a lua filter that uses the pandoc frontmatter to parse metadata and display it in documents.

As a matter of fact: I did that. See the [`mega_replacer_filter.lua`](https://github.com/Tiefseetauchner/TiefDownConverter/blob/main/docs/template/luafilters/mega_replacer_filter.lua) and [`navlib.lua`](https://github.com/Tiefseetauchner/TiefDownConverter/blob/main/docs/template/navlib.lua) for more on that. It also integrates nicely with navigation metadata, see [Navigation Metadata](#navigation-metadata).

The basic operational principles of lua filters are as follows:

1. Lua filters are defined per template.
2. Discovery runs on Lua filters, recursively entering directories.
3. If the conversion allows lua filters (i.e. the cli is pandoc), they are added to the preprocessing step automatically.

Lua filters are also added to the processing step of EPUB and CustomProcessor conversions.
