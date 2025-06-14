## `tiefdownconverter project markdown meta` {#projectmarkdownmeta}

**Version:** `tiefdownconverter 0.9.0-ALPHA.1`

### Usage:
```
Manage the metadata of a markdown project.
This metadata is markdown project specific and is not shared between projects.
This metadata takes precedence over the shared metadata.

Usage: tiefdownconverter project markdown meta [OPTIONS] <NAME> <COMMAND>

Commands:
  set     Add or change the metadata. Overrides previous keys.
  remove  Remove metadata.
  list    List the metadata.
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
- [set](#projectmarkdownmetaset)
- [remove](#projectmarkdownmetaremove)
- [list](#projectmarkdownmetalist)

