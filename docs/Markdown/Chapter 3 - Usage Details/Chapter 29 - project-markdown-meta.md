## `tiefdownconverter project markdown meta` {#projectmarkdownmeta}

**Version:** `tiefdownconverter 0.8.0-ALPHA`

### Usage:
```
Manage the metadata of a markdown project.
This metadata is markdown project specific and is not shared between projects.
This metadata takes precedence over the shared metadata.

Usage: tiefdownconverter project markdown meta <NAME> <COMMAND>

Commands:
  set     Add or change the metadata. Overrides previous keys.
  remove  Remove metadata.
  list    List the metadata.
  help    Print this message or the help of the given subcommand(s)

Arguments:
  <NAME>
          The name of the markdown project to update.

Options:
  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- [set](#projectmarkdownmetaset)
- [remove](#projectmarkdownmetaremove)
- [list](#projectmarkdownmetalist)

