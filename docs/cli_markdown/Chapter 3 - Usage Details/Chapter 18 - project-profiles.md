## `tiefdownconverter project profiles` {#projectprofiles}

**Version:** `tiefdownconverter 0.9.2-ALPHA.1`

### Usage:
```
Manage the conversion profiles of the project.
A conversion profile defines a collection of templates to be converted at the same time.
This can be used to prepare presets (for example, web export, PDF export, ...).
It can also be used for defining default templates for markdown projects.

Usage: tiefdownconverter project profiles [OPTIONS] <COMMAND>

Commands:
  add     Add a new conversion profile to the project.
  remove  Remove a conversion profile from the project.
  list    List the conversion profiles in the project.
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- [add](#projectprofilesadd)
- [remove](#projectprofilesremove)
- [list](#projectprofileslist)

