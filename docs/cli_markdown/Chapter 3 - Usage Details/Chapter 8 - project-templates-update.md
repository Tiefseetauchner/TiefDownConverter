## `tiefdownconverter project templates update` {#projecttemplatesupdate}

**Version:** `tiefdownconverter 0.8.0`

### Usage:

```
Update a template in the project.

Usage: tiefdownconverter project templates <TEMPLATE> update [OPTIONS]

Options:
      --template-file <TEMPLATE_FILE>
          The file to use as the template. If not provided, the template name will be used.

  -v, --verbose
          Enable verbose output.

      --template-type <TEMPLATE_TYPE>
          The type of the template. If not provided, the type will be inferred from the template file.
          Changing this is not recommended, as it is highly unlikely the type and only the type has changed. It is recommended to create a new template instead.

          [possible values: tex, typst, epub, custom-pandoc]

      --output <OUTPUT>
          The output file. If not provided, the template name will be used.

      --filters <FILTERS>...
          The luafilters to use for pandoc conversion of this templates markdown.
          This replaces all existing filters.

      --add-filters <ADD_FILTERS>...
          The luafilters to use for pandoc conversion of this templates markdown.
          This adds to the existing filters.

      --remove-filters <REMOVE_FILTERS>...
          The luafilters to use for pandoc conversion of this templates markdown.
          This removes the filter from the existing filters.

      --preprocessor <PREPROCESSOR>
          The preprocessor to use for this template.
          A preprocessor defines the arguments passed to the pandoc conversion from markdown.
          If using a CustomPandoc template, a preprocessor is required.
          Preprocessors replace all arguments. Thus, with preprocessors, you need to define the output file and format.
          For templates, that is the file imported by the template.
          Preprocessors are incompatible with epub conversion. Use processors instead.

      --processor <PROCESSOR>
          The processor to use for this template.
          A processor defines additional arguments passed to the conversion command.
          For LaTeX and typst templates, this allows extending the respective conversion parameters.
          For epub templates, this allows adding custom pandoc parameters.
          Processors are incompatible with CustomPandoc conversions. Use preprocessors instead.

  -h, --help
          Print help (see a summary with '-h')
```
