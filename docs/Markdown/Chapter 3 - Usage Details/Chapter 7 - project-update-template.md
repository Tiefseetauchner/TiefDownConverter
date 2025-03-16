## `tiefdownconverter project update-template`

**Version:** `tiefdownconverter 0.4.0`

### Usage:
```
Update a template in the project.

Usage: tiefdownconverter project update-template [OPTIONS] <TEMPLATE>

Arguments:
  <TEMPLATE>  The template to update.

Options:
      --template-file <TEMPLATE_FILE>
          The file to use as the template. If not provided, the template
    name will be used.
      --template-type <TEMPLATE_TYPE>
          The type of the template. If not provided, the type will be
    inferred from the template file.
          Changing this is not recommended, as it is highly unlikely the
    type and only the type has changed. It is recommended to create a new
    template instead. [possible values: tex, typst, epub]
      --output <OUTPUT>
          The output file. If not provided, the template name will be used.
      --filters <FILTERS>...
          The luafilters to use for pandoc conversion of this templates
    markdown.
      --add-filters <ADD_FILTERS>...
          The luafilters add to the template.
      --remove-filters <REMOVE_FILTERS>...
          The luafilters to remove from the template.
  -h, --help
          Print help
```

