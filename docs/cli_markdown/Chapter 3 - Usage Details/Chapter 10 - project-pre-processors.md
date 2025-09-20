## `tiefdownconverter project pre-processors` {#projectpre-processors}

**Version:** `tiefdownconverter 0.9.1-ALPHA.1`

### Usage:
```
Manage the preprocessors of the project.
A preprocessor defines the arguments passed to the pandoc conversion from markdown.
If using a CustomPreprocessor template, a preprocessor is required.
Preprocessors replace all arguments. Thus, with preprocessors, you need to define the output file and format.
For templates, that is the file imported by the template.
Preprocessors are incompatible with epub conversion. Use processors instead.

Usage: tiefdownconverter project pre-processors [OPTIONS] <COMMAND>

Commands:
  add     Add a new preprocessor to the project.
  remove  Remove a preprocessor from the project.
  list    List the preprocessors in the project.
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- [add](#projectpre-processorsadd)
- [remove](#projectpre-processorsremove)
- [list](#projectpre-processorslist)

