## `tiefdownconverter project` {#project}

**Version:** `tiefdownconverter 0.7.0-alpha`

### Usage:
```
Update the TiefDown project.

Usage: tiefdownconverter project [PROJECT] <COMMAND>

Commands:
  templates        Add or modify templates in the project.
  update-manifest  Update the project manifest.
  pre-processors   Manage the preprocessors of the project.
  processors       Manage the preprocessors of the project.
  profiles         Manage the conversion profiles of the project.
  list-templates   List the templates in the project.
  validate         Validate the TiefDown project structure and metadata.
  clean            Clean temporary files from the TiefDown project.
  smart-clean      Clean temporary files from the TiefDown project, leaving only the threshold amount of folders.
  help             Print this message or the help of the given subcommand(s)

Arguments:
  [PROJECT]  The project to edit. If not provided, the current directory will be used.

Options:
  -h, --help  Print help
```

### Subcommands:
- [templates](#projecttemplates)
- [update-manifest](#projectupdate-manifest)
- [pre-processors](#projectpre-processors)
- [processors](#projectprocessors)
- [profiles](#projectprofiles)
- [list-templates](#projectlist-templates)
- [validate](#projectvalidate)
- [clean](#projectclean)
- [smart-clean](#projectsmart-clean)

