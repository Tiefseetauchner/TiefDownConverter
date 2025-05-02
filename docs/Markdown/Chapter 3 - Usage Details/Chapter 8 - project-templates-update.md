## `tiefdownconverter project templates update` {#projecttemplatesupdate}

**Version:** `tiefdownconverter 0.8.0-ALPHA`

### Usage:
```
Update a template in the project.

Usage: tiefdownconverter project templates <TEMPLATE> update [OPTIONS]

Options:
      --template-file <TEMPLATE_FILE>
          The file to use as the template. If not provided, the template name will be used.
      --template-type <TEMPLATE_TYPE>
          The type of the template. If not provided, the type will be inferred from the template file.
          Changing this is not recommended, as it is highly unlikely the type and only the type has changed. It is recommended to create a new template instead. [possible values: tex, typst, epub, custom-pandoc]
      --output <OUTPUT>
          The output file. If not provided, the template name will be used.
      --filters <FILTERS>...
          The luafilters to use for pandoc conversion of this templates markdown.
      --add-filters <ADD_FILTERS>...
          The luafilters add to the template.
      --remove-filters <REMOVE_FILTERS>...
          The luafilters to remove from the template.
      --preprocessor <PREPROCESSOR>
          The preprocessor to use for this template.
      --processor <PROCESSOR>
          The processor to use for this template.
  -h, --help
          Print help
```

