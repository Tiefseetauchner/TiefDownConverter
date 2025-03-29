## `tiefdownconverter project templates add` {#projecttemplatesadd}

**Version:** `tiefdownconverter 0.7.0-alpha`

### Usage:
```
Add a new template to the project.

Usage: tiefdownconverter project templates <TEMPLATE> add [OPTIONS]

Options:
  -f, --template-file <TEMPLATE_FILE>  The file to use as the template. If not provided, the template name will be used.
  -t, --template-type <TEMPLATE_TYPE>  The type of the template. If not provided, the type will be inferred from the template file. [possible values: tex, typst, epub, custom-pandoc]
  -o, --output <OUTPUT>                The output file. If not provided, the template name will be used.
      --filters <FILTERS>...           The luafilters to use for pandoc conversion of this templates markdown.
      --preprocessor <PREPROCESSOR>    The preprocessor to use for this template.
      --processor <PROCESSOR>          The processor to use for this template.
  -h, --help                           Print help
```

