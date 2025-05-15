# Templates

Templating in TiefDown is done in several ways:

- [LaTeX templates](#latex-templates)
  - The most basic form of templating, it generates a LaTeX document from the
    markdown files that can be included in a LaTeX document.
  - Supports Metadata file generation.
- [Typst templates](#typst-templates)
  - Similar to LaTeX, it generates a Typst document from the markdown files
    that can be included in a Typst document.
  - Supports Metadata file generation.
- [EPUB templates](#epub-templates)
  - A legacy templating system, it generates a EPUB document from the markdown
    files. Convoluted and very much custom to basic usage.
  - Adds Metadata directly to the EPUB file.
  - (!) This template type should be forgone in favour of Custom Pandoc
    Conversion.
- [Custom pandoc conversion](#custom-pandoc-conversion)
  - A more advanced templating system, it runs custom pandoc commands on the
    markdown files. This is the most flexible templating system, but also the
    most complex.
  - Supports Metadata insertion into command line arguments.

## LaTeX Templates

LaTeX templates are the most intuitive form of templating in TiefDown, but also
the most fleshed out. The basic usage generates a LaTeX document from markdown,
usually output.tex, with [lua-filters](#lua-filters) applied depending on the
template.
