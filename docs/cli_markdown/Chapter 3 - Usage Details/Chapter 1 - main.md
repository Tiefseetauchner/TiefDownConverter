## `tiefdownconverter ` {#main}

**Version:** `tiefdownconverter 0.9.0-ALPHA.1`

### Usage:
```
TiefDownConverter manages TiefDown projects.
TiefDown is a project structure meant to simplify the conversion process from Markdown to PDFs.
TiefDownConverter consolidates multiple conversion processes and templating systems to generate a configurable set or subset of output documents.
It is not in itself a converter, but a wrapper around pandoc, xelatex and typst. As such, it requires these dependencies to be installed.

Usage: tiefdownconverter [OPTIONS] <COMMAND>

Commands:
  convert             Convert a TiefDown project. By default, it will convert the current directory.
  init                Initialize a new TiefDown project.
  project             Update the TiefDown project.
  check-dependencies  Validate dependencies are installed.
  help                Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Subcommands:
- [convert](#convert)
- [init](#init)
- [project](#project)
- [check-dependencies](#check-dependencies)

