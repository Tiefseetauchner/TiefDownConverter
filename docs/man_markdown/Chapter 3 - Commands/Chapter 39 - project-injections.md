## tiefdownconverter project injections

```
Manage the injections of the project.
An injection defines an additional, template scoped mechanism for adding files to the combined output of the preprocessors.
Each injection can have multiple files or directories associated with it.
An injection can be used in three ways:
- Header injections: Get preprended to the document in the order in which they are listed in the manifest.
- Body injections: Get inserted and sorted in the primary document.
- Footer injections: Get appended to the document in the order in which they are listed in the manifest.

Usage: tiefdownconverter project injections [OPTIONS] <COMMAND>

Commands:
  create     Creates a new injection.
  remove     Removes an injection.
  add-files  Adds files to an injection.
  help       Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
          Enable verbose output.

  -h, --help
          Print help (see a summary with '-h')
```

### Subcommands:
- create
- remove
- add-files

