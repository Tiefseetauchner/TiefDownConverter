## `tiefdownconverter project markdown` {#projectmarkdown}

**Version:** `tiefdownconverter 0.9.2-ALPHA.1`

### Usage:
```
Manage the markdown projects of the project.
A markdown project defines the markdown conversion process for a project.
There can be multiple markdown projects with different markdown files.
Each markdown project also has a seperate output folder ('.' per default).
A markdown project can have seperate metadata.
A markdown project can have resources that are copied to the respective conversion folder.

Usage: tiefdownconverter project markdown [OPTIONS] <COMMAND>

Commands:
  add        Add a new markdown project to the project.
  update     Update a markdown project in the project.
  meta       Manage the metadata of a markdown project.
  resources  Manage the resources of a markdown project.
  remove     Remove a markdown project from the project.
  list       List the markdown projects in the project.
  help       Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- [add](#projectmarkdownadd)
- [update](#projectmarkdownupdate)
- [meta](#projectmarkdownmeta)
- [resources](#projectmarkdownresources)
- [remove](#projectmarkdownremove)
- [list](#projectmarkdownlist)

