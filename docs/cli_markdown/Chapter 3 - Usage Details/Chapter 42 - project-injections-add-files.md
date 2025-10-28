## `tiefdownconverter project injections add-files` {#projectinjectionsadd-files}

**Version:** `tiefdownconverter 0.9.2-ALPHA.1`

### Usage:
```
Adds files to an injection.

Usage: tiefdownconverter project injections add-files [OPTIONS] <NAME> [FILES]...

Arguments:
  <NAME>
          The name of the injection to modify.

  [FILES]...
          The files to be added to the injection.
          Can be a directory.
          The order of the files here defines the order for header and footer injections.
          For body injections, the files are ordered as per the default algorithm.
          Files in directories are ordered as per the default algorithm.
          Duplicate files will be added twice.

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

