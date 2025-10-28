## `tiefdownconverter project markdown resources` {#projectmarkdownresources}

**Version:** `tiefdownconverter 0.9.2-ALPHA.1`

### Usage:
```
Manage the resources of a markdown project.
Resources are a way to include meta information and resources on a per project basis.
This is helpful for example for including a custom css file for a project, as that is not possible purely with metadata.
Resources are stored in the markdown folder and copied to the conversion directory for that profile before conversion.

Usage: tiefdownconverter project markdown resources [OPTIONS] <NAME> <COMMAND>

Commands:
  add     Add a new resource to the project.
  remove  Remove a resource from the project.
  list    List the resources in the project.
  help    Print this message or the help of the given subcommand(s)

Arguments:
  <NAME>
          The name of the markdown project to update.

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- [add](#projectmarkdownresourcesadd)
- [remove](#projectmarkdownresourcesremove)
- [list](#projectmarkdownresourceslist)

