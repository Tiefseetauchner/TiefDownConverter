## tiefdownconverter project pre-processors add

```
Add a new preprocessor to the project.

Usage: tiefdownconverter project pre-processors add [OPTIONS] <NAME> <COMBINED_OUTPUT> [-- <CLI_ARGS>...]

Arguments:
  <NAME>
          The name of the preprocessor to create.

  <COMBINED_OUTPUT>
          The file the input gets converted to.
          When preprocessing the input files, the files will get converted, combined and written this filename.

  [CLI_ARGS]...
          The arguments to pass to the preprocessor.

Options:
      --cli <CLI>
          The program to use as the preprocessor.
          Requires cli arguments
          Should Pandoc not be the required preprocessor for your use case, you can change the called cli program.

  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

