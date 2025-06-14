## `tiefdownconverter project pre-processors add` {#projectpre-processorsadd}

**Version:** `tiefdownconverter 0.8.0`

### Usage:

```
Add a new preprocessor to the project.

Usage: tiefdownconverter project pre-processors add [OPTIONS] <NAME> [-- <PANDOC_ARGS>...]

Arguments:
  <NAME>
          The name of the preprocessor to create.

  <COMBINED_OUTPUT>
          The file the input gets converted to.
          When preprocessing the input files, the files will get converted, combined and written this filename.

  [PANDOC_ARGS]...
          The arguments to pass to the preprocessor.

Options:
  -v, --verbose  Enable verbose output.
  -h, --help     Print help
```
