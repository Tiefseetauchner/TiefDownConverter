## tiefdownconverter project pre-processors add

```
Add a new preprocessor to the project.

Usage: tiefdownconverter project pre-processors add [OPTIONS] <NAME> <COMBINED_OUTPUT> [-- <PANDOC_ARGS>...]

Arguments:
  <NAME>
          The name of the preprocessor to create.

  <COMBINED_OUTPUT>
          The file the input gets converted to.
          When preprocessing the input files, the files will get converted, combined and written this filename.

  [PANDOC_ARGS]...
          The arguments to pass to the preprocessor.

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

