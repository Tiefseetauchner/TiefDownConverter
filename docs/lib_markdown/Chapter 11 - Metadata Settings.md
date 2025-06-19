# Metadata Settings {#metadata-settings}

Metadata settings influence how metadata files are generated. The
`[metadata_settings]` table currently supports the `metadata_prefix` option.
This prefix determines the name of the macro or object used to access
metadata in templates.

For example, with

```toml
[metadata_settings]
metadata_prefix = "book"
```

the generated LaTeX file defines a `\book{}` command while Typst exposes a
`book` object. In other words, the prefix fully replaces the default `meta`
name. If no prefix is set the command and object are called `meta`.
