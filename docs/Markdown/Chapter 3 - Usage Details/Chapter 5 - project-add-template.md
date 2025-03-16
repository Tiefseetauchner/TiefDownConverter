## `tiefdownconverter project add-template`

**Version:** `tiefdownconverter 0.4.0`

### Usage:
```
Add a new template to the project.

Usage: tiefdownconverter project add-template [OPTIONS] <TEMPLATE>

Arguments:
  <TEMPLATE>  The name of the template to create. If using a LiX template,
    make sure to install the corresponding .sty and .cls files from
    https://github.com/NicklasVraa/LiX. Adjust the metadata in
    template/meta.tex accordingly.

Options:
  -f, --template-file <TEMPLATE_FILE>  The file to use as the template. If
    not provided, the template name will be used.
  -t, --template-type <TEMPLATE_TYPE>  The type of the template. If not
    provided, the type will be inferred from the template file. [possible
    values: tex, typst, epub]
  -o, --output <OUTPUT>                The output file. If not provided,
    the template name will be used.
      --filters <FILTERS>...           The luafilters to use for pandoc
    conversion of this templates markdown.
  -h, --help                           Print help
```

