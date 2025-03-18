## `tiefdownconverter init` {#init}

**Version:** `tiefdownconverter 0.5.0`

### Usage:
```
Initialize a new TiefDown project.

Usage: tiefdownconverter init [OPTIONS] [PROJECT]

Arguments:
  [PROJECT]  The project to initialize. If not provided, the current
    directory will be used.

Options:
  -t, --templates <TEMPLATES>...     The preset templates to use. If not
    provided, the default template.tex will be used.
                                     For custom templates, use the update
    command after initializing the project.
                                     If using a LiX template, make sure to
    install the corresponding .sty and .cls files from
    https://github.com/NicklasVraa/LiX. Adjust the metadata in
    template/meta.tex accordingly.
                                      [possible values: template.tex,
    booklet.tex, lix_novel_a4.tex, lix_novel_book.tex, template_typ.typ,
    default_epub]
  -n, --no-templates                 Do not include the default templates.
    You will need to add templates manually with Update
  -f, --force                        Delete the project if it already
    exists.
  -m, --markdown-dir <MARKDOWN_DIR>  The directory where the Markdown files
    are located. If not provided, Markdown/ will be used.
  -h, --help                         Print help
```

