## `tiefdownconverter project shared-meta` {#projectshared-meta}

**Version:** `tiefdownconverter 0.8.0-ALPHA`

### Usage:
```
Manage the shared metadata of the project.
This Metadata is shared between all markdown projects.
When converting, it is merged with the markdown project specific metadata.
When using the same key for shared and project metadata, the project metadata overrides the shared metadata.

Usage: tiefdownconverter project shared-meta <COMMAND>

Commands:
  set     Add or change the metadata. Overrides previous keys.
  remove  Remove metadata.
  list    List the metadata.
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- [set](#projectshared-metaset)
- [remove](#projectshared-metaremove)
- [list](#projectshared-metalist)

