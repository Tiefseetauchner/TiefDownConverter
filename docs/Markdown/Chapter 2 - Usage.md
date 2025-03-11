# Usage

The basic usage of `tiefdownconverter` is relatively simple.
The difficult part is understanding the templating system and
how to customise it for your usecases. Presets can only do so much.

## Installation

Currently the only way to install `tiefdownconverter` is to either build
it yourself or download a precompiled binary from the
[releases page](https://github.com/tiefseetauchner/tiefdownconverter/releases).
Then just add it to the path and you're good to go. You can of course
also just call it relatively by placing the binary in your project folder or
something like that.

There are a few dependencies that you need to install.

- [Pandoc](https://pandoc.org/): Conversion from Markdown to TeX, Typst and
  Epub.
- A TeX distribution: For converting TeX files to PDF. It has to include
  xelatex.
  - If using [TeX Live](https://www.tug.org/texlive/) you may need to
    additionally install `texlive-xetex` depending on your system.
  - If using [MikTeX](https://miktex.org/), no need to do anything.
- [Typst](https://typst.app/): For converting Typst files to PDF.

Now you should be able to run `tiefdownconverter` from the command line.
You can test it by initialising a test project using `tiefdown init testproject`
and running `tiefdown convert` in the project directory or
`tiefdown convert -p testproject`.

## Getting started

First off, you need to create a project using `tiefdownconverter init`. This will
create a new project **in the current directory**. You can (and maybe should)
specify a project using the -p flag.

This command creates the basic template structure like so:

```
your_project/
├── Markdown/
│   └── Chapter 1 - Introduction.md
├── template/
│   ├── meta.tex
│   └── template.tex
└── manifest.toml
```

The Markdown folder contains an example Markdown file. When placing your markdown files
in this folder, make sure they're named like `Chapter X.md`, with anything following the
number being ignored. *This is important*, as the converter will use this to sort the
files for conversion, as otherwise it'd have no idea in which order they should be
converted.

Now you should be able to run `tiefdownconverter convert -p path/to/your_project` (or
ommitting the -p flag if you're already in the project directory) and it should
generate a PDF file in the project directory. You can now adjust the template, add
your own Markdown files, and so on.

## Adjusting the markdown directory

You can change what directory the converter looks for markdown files in by changing the
`markdown_dir` field in the manifest.toml file or saying `-m path/to/markdown/dir` when
initialising the project. You can also change it post-initialisation using
`tiefdownconverter project update-manifest -m path/to/markdown/dir`. If you don't do so,
the converter will look for markdown files in the `project_dir/Markdown` directory.

## Customising the template

The key idea behind tiefdownconverter is, that it can handle multiple templates at the
same time. This is done by creating a template file in the template directory and adding
it to the project's manifest.toml file.

You could do this manually, if you were so inclined, but using 
`tiefdownconverter project add-template` is much easier. Check the 
[Usage Details](#usage-details) for the usage of this command. But importantly, once you
created the template and added it to the manifest, you will be able to convert using it.
`tiefdownconverter convert -p path/to/your_project --templates <TEMPLATE_NAME>` will convert
only the selected template, aiding in debugging.

And now, you're pretty much free to do whatever you want with the template. Write tex or typst
templates, use custom filters, so on.

## Adjusting template behaviour

You have a few options for editing template behaviour using `tiefdownconverter`. You can of
course edit the template files directly, but there are a few more options.

Mainly and most interestingly, lua filters can adjust the behaviour of the markdown conversion.
These are lua scripts that are run before the markdown is converted to tex or typst. You can
add lua filters to a template by either editing the manifest or using 
`tiefdownconverter project update-template <TEMPLATE_NAME> --add-filters <FILTER_NAME>`. This
can be either the path to a lua filter (relative to the project directory) or a directory
containing lua filters.

You can also change the name of the exported file by setting the `output` option. For example,
`tiefdownconverter project update-template <TEMPLATE_NAME> --output <NEW_NAME>`. This will
export the template to `<NEW_NAME>.pdf` instead of the default `<TEMPLATE_NAME>.pdf`.

Similarly, you could change the template file and type, though I advice against it, as this
may break the template. I advice to just add a new template and remove the old one using
`tiefdownconverter project remove-template <TEMPLATE_NAME>`.

## Writing filters

> **Note:** This section only really addresses LaTeX, but the concepts are the same for
> Typst.

If you are in the business of writing filters (and don't just solve everything in TeX itself), 
I advice checking out the documentation at 
[https://pandoc.org/lua-filters.html](https://pandoc.org/lua-filters.html). But here's a
quick rundown of what you can do. For example, if you wanted to change the font of all
block quotes, there's a few things you'd need to do. First off, in your template, you will 
need to define a font. It could look something like this:

```tex
\usepackage{fontspec}
\newfontfamily\blockquotefont{Noto Sans}
```

Then, add a filter to your template as described above. The filter could look something like this:
```lua
function BlockQuote(el)
  local tt_start = pandoc.RawBlock('latex', '\\blockquotefont\\small')

  table.insert(el.content, 1, tt_start)

  return el
end
```

Of course, you could just redefine the font in TeX but I think this is a bit more flexible. One
usecase that is quite important is to change the way chapters are handled for LiX. In case of
LiX, they expect `\h{chapter_name}` instead of `\section`, which is the standard behaviour of
pandoc. So when you create a LiX backed template, you have to add a filter to change that
behaviour. Something like this:

```lua
function Header(elem)
  if elem.level == 1 then
    return pandoc.RawBlock("latex", "\\h{" .. pandoc.utils.stringify(elem.content) .. "}")
  end
  if elem.level == 2 then
    return pandoc.RawBlock("latex", "\\hh{" .. pandoc.utils.stringify(elem.content) .. "}")
  end
  -- add more levels here if needed
end
```

