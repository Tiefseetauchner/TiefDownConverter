## `tiefdownconverter project templates add` {#projecttemplatesadd}

**Version:** `tiefdownconverter 0.9.0-ALPHA.1`

### Usage:
```
Add a new template to the project.
If using a preset template name, the preset will be copied to the template folder.
If using a custom template, make sure to add the respective files to the template folder.
Available preset templates are: template.tex, booklet.tex, lix_novel_a4.tex, lix_novel_book.tex, template_typ.typ, default_epub

Usage: tiefdownconverter project templates <TEMPLATE> add [OPTIONS]

Options:
  -f, --template-file <TEMPLATE_FILE>
          The file to use as the template. If not provided, the template name will be used.

  -v, --verbose
          Enable verbose output.

  -t, --template-type <TEMPLATE_TYPE>
          The type of the template. If not provided, the type will be inferred from the template file.
          
          [possible values: tex, typst, epub, custom-preprocessors, custom-processor]

  -o, --output <OUTPUT>
          The output file. If not provided, the template name will be used.

      --filters <FILTERS>...
          The luafilters to use for pandoc conversion of this templates markdown.
          Luafilters are lua scripts applied during the pandoc conversion.
          You can add a folder or a filename. If adding a folder, it will be traversed recursively, and any .lua file will be added.
          See the pandoc documentation and 'Writing filters' of the TiefDownConverter documentation for more details.

      --preprocessors <PREPROCESSORS>
          The preprocessors to use for this template.
          A preprocessor defines the arguments passed to the pandoc conversion from the specified input format.
          Each input format can have at most one preprocessor. Multiple preprocessors for the same input format will lead to an error.
          There can be a preprocessor without an input format, which will be used if no other preprocessor matches the input format. Only one such preprocessor is allowed.
          If using a CustomPreprocessors template, at least one preprocessor is required.
          Preprocessors replace all arguments. Thus, with preprocessors, you need to define the output file and format.
          For templates, that is the file imported by the template.

      --preprocessor-output <PREPROCESSOR_OUTPUT>
          The output file of the preprocessor. If not provided, the template name with the appropriate ending will be used.
          This is the file the input gets converted to. When preprocessing the input files, the files will get converted, combined and written to this filename.

      --processor <PROCESSOR>
          The processor to use for this template.
          A processor defines additional arguments passed to the conversion command.
          For LaTeX and typst templates, this allows extending the respective conversion parameters.
          Processors are incompatible with CustomPreprocessors conversions. Use preprocessors instead.

  -h, --help
          Print help (see a summary with '-h')
```

