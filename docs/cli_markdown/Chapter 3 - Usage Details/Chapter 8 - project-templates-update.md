## `tiefdownconverter project templates update` {#projecttemplatesupdate}

**Version:** `tiefdownconverter 0.9.2-ALPHA.2`

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
          
          [possible values: tex, typst, epub, custom-preprocessors, custom-processor]

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

      --preprocessors <PREPROCESSORS>
          The preprocessors to use for this template.
          A preprocessor defines the arguments passed to the pandoc conversion from the specified input format.
          Each input format can have at most one preprocessor. Multiple preprocessors for the same input format will lead to an error.
          There can be a preprocessor without an input format, which will be used if no other preprocessor matches the input format. Only one such preprocessor is allowed.
          If using a CustomPreprocessor template, at least one preprocessor is required.
          Preprocessors replace all arguments. Thus, with preprocessors, you need to define the output file and format.
          For templates, that is the file imported by the template.

      --add-preprocessors <ADD_PREPROCESSORS>...
          The preprocessors to use for this template.
          This adds to the existing preprocessors.

      --remove-preprocessors <REMOVE_PREPROCESSORS>...
          The preprocessors to use for this template.
          This removes the preprocessor from the existing preprocessors.

      --preprocessor-output <PREPROCESSOR_OUTPUT>
          The output file of the preprocessor. If not provided, the template name with the appropriate ending will be used.
          This is the file the input gets converted to. When preprocessing the input files, the files will get converted, combined and written to this filename.

      --processor <PROCESSOR>
          The processor to use for this template.
          A processor defines additional arguments passed to the conversion command.
          For LaTeX and typst templates, this allows extending the respective conversion parameters.
          Processors are incompatible with CustomPreprocessor conversions. Use preprocessors instead.

      --header-injections <HEADER_INJECTIONS>...
          The injection to use for prepending to the preprocessing step.
          A header injection can define one or more files that will be prepended to the preprocessing step.
          Files in header injections get prepended in the order that they are defined in in the manifest.
          Duplicate files will be added twice.
          Injections have to be defined in the manifest.

      --body-injections <BODY_INJECTIONS>...
          The injection to use for inserting into the preprocessing step.
          A body injection can define one or more files that will be inserted into the preprocessing step.
          Files in body injections get inserted in accordance with the sorting algorithm.
          Duplicate files will be added twice.
          Injections have to be defined in the manifest.

      --footer-injections <FOOTER_INJECTIONS>...
          The injection to use for appending to the preprocessing step.
          A footer injection can define one or more files that will be appended to the preprocessing step.
          Files in header injections get appended in the order that they are defined in in the manifest.
          Duplicate files will be added twice.
          Injections have to be defined in the manifest.

  -h, --help
          Print help (see a summary with '-h')
```

