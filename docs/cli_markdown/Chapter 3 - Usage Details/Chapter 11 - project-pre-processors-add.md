## `tiefdownconverter project pre-processors add` {#projectpre-processorsadd}

**Version:** `tiefdownconverter 0.9.0`

### Usage:
```
Add a new preprocessor to the project.

Usage: tiefdownconverter project pre-processors add [OPTIONS] <NAME> [-- <CLI_ARGS>...]

Arguments:
  <NAME>
          The name of the preprocessor to create.

  [CLI_ARGS]...
          The arguments to pass to the preprocessor.

Options:
      --filter <FILTER>
          The file extension filter for the preprocessor.
          This defines which input files the preprocessor is applied to. If not provided, the preprocessor will be applied to all input files.
          Allows glob patterns. Excludes the leading dot. Only matches the file extension.

  -v, --verbose
          Enable verbose output.

      --cli <CLI>
          The program to use as the preprocessor.
          Requires cli arguments
          Should Pandoc not be the required preprocessor for your use case, you can change the called cli program.

  -h, --help
          Print help (see a summary with '-h')
```

