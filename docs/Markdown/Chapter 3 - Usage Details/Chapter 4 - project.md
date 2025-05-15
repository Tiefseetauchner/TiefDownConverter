## `tiefdownconverter project` {#project}

**Version:** `tiefdownconverter 0.8.1`

### Usage:
```
Update the TiefDown project.

Usage: tiefdownconverter project [OPTIONS] [PROJECT] <COMMAND>

Commands:
  templates        Add or modify templates in the project.
  update-manifest  Update the project manifest.
  pre-processors   Manage the preprocessors of the project.
  processors       Manage the processors of the project.
  profiles         Manage the conversion profiles of the project.
  shared-meta      Manage the shared metadata of the project.
  markdown         Manage the markdown projects of the project.
  list-templates   List the templates in the project.
  validate         Validate the TiefDown project structure and metadata.
                   NOTE: This command is deprecated and will be removed in a future release. It's pointless and a maintenance nightmare.
  clean            Clean temporary files from the TiefDown project.
  smart-clean      Clean temporary files from the TiefDown project, leaving only the threshold amount of folders.
  help             Print this message or the help of the given subcommand(s)

Arguments:
  [PROJECT]  The project to edit. If not provided, the current directory will be used.

Options:
  -v, --verbose  Enable verbose output.
  -h, --help     Print help
```

### Subcommands:
- [templates](#projecttemplates)
- [update-manifest](#projectupdate-manifest)
- [pre-processors](#projectpre-processors)
- [processors](#projectprocessors)
- [profiles](#projectprofiles)
- [shared-meta](#projectshared-meta)
- [markdown](#projectmarkdown)
- [list-templates](#projectlist-templates)
- [validate](#projectvalidate)
- [clean](#projectclean)
- [smart-clean](#projectsmart-clean)

