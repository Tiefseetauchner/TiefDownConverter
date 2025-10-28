## `tiefdownconverter project processors` {#projectprocessors}

**Version:** `tiefdownconverter 0.9.2-ALPHA.1`

### Usage:
```
Manage the processors of the project.
A processor defines additional arguments passed to the conversion command.
For LaTeX and typst templates, this allows extending the respective conversion parameters.
For CustomProcessor templates, this allows adding custom pandoc parameters.
Processors are incompatible with CustomPreprocessors conversions. Use preprocessors instead.

Usage: tiefdownconverter project processors [OPTIONS] <COMMAND>

Commands:
  add     Add a new processor to the project.
  remove  Remove a processor from the project.
  list    List the processors in the project.
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- [add](#projectprocessorsadd)
- [remove](#projectprocessorsremove)
- [list](#projectprocessorslist)

