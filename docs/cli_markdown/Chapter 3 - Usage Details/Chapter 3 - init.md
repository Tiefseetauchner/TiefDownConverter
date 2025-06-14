## `tiefdownconverter init` {#init}

**Version:** `tiefdownconverter 0.8.0`

### Usage:

```
Initialize a new TiefDown project.

Usage: tiefdownconverter init [OPTIONS] [PROJECT]

Arguments:
  [PROJECT]
          The project to initialize. If not provided, the current directory will be used.

Options:
  -t, --templates <TEMPLATES>...
          The preset templates to use. If not provided, the default template.tex will be used.
          For custom templates, use the update command after initializing the project.
          If using a LiX template, make sure to install the corresponding .sty and .cls files from https://github.com/NicklasVraa/LiX. Adjust the metadata in template/meta.tex accordingly.


          [possible values: template.tex, booklet.tex, lix_novel_a4.tex, lix_novel_book.tex, template_typ.typ, default_epub]

  -v, --verbose
          Enable verbose output.

  -n, --no-templates
          Do not include the default templates. You will need to add templates manually with Update

  -f, --force
          Delete the project if it already exists.

  -m, --markdown-dir <MARKDOWN_DIR>
          The directory where the Markdown files are located. If not provided, Markdown/ will be used.

      --smart-clean
          Enables smart clean for the project with a default threshold of 5.
          If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders.

      --smart-clean-threshold <SMART_CLEAN_THRESHOLD>
          The threshold for smart clean. If not provided, the default threshold of 5 will be used.
          If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders.

  -h, --help
          Print help (see a summary with '-h')
```
