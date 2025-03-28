# Usage

The basic usage of `tiefdownconverter` is relatively simple.
The difficult part is understanding the templating system and
how to customise it for your usecases. Presets can only do so much.

## Installation

Currently the only way to install `tiefdownconverter` is to either build
it yourself, install it from cargo, or download a precompiled binary from the
[releases page](https://github.com/tiefseetauchner/tiefdownconverter/releases).
Then just add it to the path and you're good to go. You can of course
also just call it relatively by placing the binary in your project folder or
something like that.

If you build from source, run `cargo build [--release]` or `cargo install --path .`.

That said, the recommended way to install TiefDownConverter is 
`cargo install tiefdownconverter`. This will always install the latest version.

Downloading from the release is as simple as downloading the appropriate version
(Windows, Mac, Linux) and adding it to a folder in the path. You could also add
tiefdownconverter to a folder and run it from there.

There are a few dependencies that you need to install.

- [Pandoc](https://pandoc.org/): Conversion from Markdown to TeX, Typst and
  Epub.
- A TeX distribution: For converting TeX files to PDF. It has to include
  xelatex.
  - If using [TeX Live](https://www.tug.org/texlive/) you may need to
    additionally install `texlive-xetex` depending on your system.
  - If using [MikTeX](https://miktex.org/), no need to do anything.
- [Typst](https://typst.app/): For converting Typst files to PDF.

Windows is easy enough: `winget install miktex pandoc typst`.

Linux varies by distro of course, but for ubuntu it's `apt install texlive-xetex pandoc` 
and `cargo install typst` or downloading the typst binary and adding it to the path.

Mac is still to be tested, but MacTex should have XeTeX installed.

Now you should be able to run `tiefdownconverter` from the command line.
You can test it by initialising a test project using `tiefdownconverter init testproject`
and running `tiefdownconverter convert` in the project directory or
`tiefdownconverter convert -p testproject`. You could also, as a test, clone the
[Github Repo](https://github.com/Tiefseetauchner/TiefDownConverter) and run
`tiefdownconverter convert -p docs` (this may however throw warnings if you don't
have the appropriate fonts installed).

## Getting started

TL;DR: Make a folder, go into it and run `tiefdownconverter init` and 
`tiefdownconverter convert`. That's it.

Long anser: First off, you need to create a project using `tiefdownconverter init`. This will
create a new project **in the current directory**. You can (and maybe should)
specify a project.

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
export the template to `<NEW_NAME>` instead of the default `<TEMPLATE_NAME>.pdf`.

Similarly, you could change the template file and type, though I advice against it, as this
may break the template. I advice to just add a new template and remove the old one using
`tiefdownconverter project remove-template <TEMPLATE_NAME>`.

## Conversion Profiles

A conversion profile is a shortcut to defining templates for the conversion. If you're dealing with
a lot of templates, you may be considering only converting some at any time - for example, web ready
PDFs vs. print ready PDFs, or only converting a certain size of PDF.

For that, there are conversion profiles which simply are a list of templates. It's essentially like
saving your --templates arguments.

You can create these profiles with the `project add-profile` command, setting a name and a comma
seperated list of templates. Removing a profile is also possible with the `project remove-profile`
command.

Running a conversion with a profile is as simple as adding the `--profile` flag.

The manifest file can optionally contain a section for this, if you desire to configure them
manually:

```toml
[[profiles]]
name = "PDF"
templates = ["PDF Documentation LaTeX", "PDF Documentation"]
```

## Writing templates

Importantly, when you write your own template, you need to include the content somehow.
That somehow is done via `\input{output.tex}` or `#include "./output.typ"`. This will include the 
output of the Markdown conversion in your template file. If you're using custom preprocessors, you
can change the output file of the conversion. See [Preprocessing](#preprocessing) for more
information.

EPUB support in TiefDownConverter isn’t as fancy as LaTeX or Typst, but you can still tweak it to 
look nice. You don’t get full-blown templates, but you can mess with CSS, fonts, and Lua filters 
to make it work how you want.

### Customizing CSS
EPUBs use stylesheets to control how everything looks. The good news? Any `.css` file you drop into 
`template/my_epub_template/` gets automatically loaded. No need to mess with the manifest - just 
throw in your styles and you’re good.

Example CSS:
```css
body {
    font-family: "Noto Serif", serif;
    line-height: 1.6;
    margin: 1em;
}
blockquote {
    font-style: italic;
    border-left: 3px solid #ccc;
    padding-left: 10px;
}
```

### Adding Fonts
Fonts go into `template/my_epub_template/fonts/`, and TiefDownConverter will automatically pick them up. To use them, you just need to reference them properly in your CSS:

```css
@font-face {
  font-family: 'EB Garamond';
  font-style: normal;
  font-weight: normal;
  src: url('../fonts/EBGaramond-Regular.ttf');
}

body {
    font-family: "EB Garamond", serif;
}
```

### Metadata and Structure
EPUBs need some basic metadata, which you define in the YAML front matter of your Markdown files. Stuff like title, author, and language goes here:

```yaml
---
title:
- type: main
  text: "My Publication"
- type: subtitle
  text: "A tale of loss and partying hard"
creator:
- role: author
  text: Your Name
rights: "Copyright © 2012 Your Name"
---
```

This makes sure your EPUB doesn’t look like a nameless file when opened in an e-reader.

### Using Lua Filters
Want to tweak the structure? That’s what Lua filters are for. You can use them to rename chapters, remove junk, or modify how elements are processed.
markdown_dir = "Custom Markdown Directory"
version = 1

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"

Example: Automatically renaming chapter headers:
```lua
function Header(el)
  if el.level == 1 then
    return pandoc.Header(el.level, "Chapter: " .. pandoc.utils.stringify(el.content))
  end
end
```

And that’s it. You get a customized EPUB without having to fight with the defaults. Enjoy!

## Conversion Engines

There are currently four ways to convert your Markdown files. All of them are based on the same
system. The main difference is the output format and the program it gets converted with.

### LaTeX

LaTeX is the best supported by TiefDownConverter, with the most presets. But as TiefDownConverter 
is a general-purpose Markdown to PDF converter, the format doesn't matter. LaTeX provides the 
highest degree of customization, making it ideal for structured documents, novels, and academic papers.

The primary way to interact with LaTeX is through templates. Lua filters and such are secondary, but an
important part of the conversion process to adjust behavior for different document classes.

### Typst

Typst is another supported engine, offering a more modern alternative to LaTeX with a simpler syntax and 
automatic layout adjustments. TiefDownConverter allows you to specify Typst templates in the project manifest.

Typst templates work similarly to LaTeX templates but are easier to modify if you need structured documents 
without deep LaTeX knowledge.

As far as I could tell, typst templates are also far more adherent to the general typst syntax, so Lua filters
are not as important. But they can still be used to adjust the output, especially for more advanced use cases.

### EPUB

TiefDownConverter also supports EPUB conversion, making it suitable for e-book generation. The conversion 
process uses Pandoc to transform the Markdown content into EPUB, applying any Lua filters defined in the manifest.

This however does not really support much in the way of templating. Customization should be done primarily via
Lua filters. Custom preprocessors are currently not supported at all.

However, you can still get some customization by including CSS and font files in your template folder. That's
the reason epub has to have a folder in the first place, so you can place CSS and font files in there.
Of course you can add multiple epub templates, but I don't know why you would want to.

EPUB output is particularly useful for digital publishing, ensuring compatibility with e-readers
and mobile devices.

### Custom Pandoc Converter

Okay. Stick with me here. The idea is, you are already converting my Markdown files with Pandoc, why not let
me convert them to whatever format? Well, this is where Custom Pandoc Conversion comes in. This long
fabled feature is the most complicated one, and you need a deep understanding of how TiefDownConverter works
and at least the ability to read Pandoc's documentation to even use it.
But if you're willing to put in the effort, you can do some pretty cool things.

The basic idea is, just, let the user decide what pandoc does. The result is chaos.

I'm being facetious, but this is actually the most powerful way to customize the
output. You add a preprocessor as described in [Preprocessing](#preprocessing) and
set the output path of the preprocessor and template to the same path. Then you can
do whatever pandoc allows. Want to convert to RTF? No issue. But beware:
you need to actually understand what's going on, otherwise you'll end up in
implementation hell.

## Writing filters

> **Note:** This section only really addresses LaTeX, but the concepts are the same for
> Typst and epub.

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

## Preprocessing

A "Preprocessor" is a stupid word for defining your own pandoc conversion parameters. You can
(and should) use this to adjust the behaviour of the converter. For example, you could
define a preprocessor to add `--listings` to the pandoc command. This is useful if you want
to have reasonable code output in your pdf.

If no preprocessor is defined, the converter will use default pandoc parameters, converting
to the intermediate output file (in case of LaTeX, this is `output.tex`). But if you for example
are using lua filters, you may want to export to a different path. This can be done by defining
a preprocessor.

If you want to define a preprocessor, you can do so by running

```bash
tiefdownconverter project update-template <TEMPLATE_NAME> --preprocessor <PREPROCESSOR_NAME>
```

 to assign it to a template and 

```bash
tiefdownconverter project add-preprocessor <PREPROCESSOR_NAME> -- [PANDOC_ARGS]
```
 to assign it to a template and 

to create a new preprocessor.

For example, if you want to add `--listings` to the pandoc command, you could do so by adding
`--listings` to the preprocessor. But importantly, **this overwrites the default preprocessor**.
So you will have to add the `-o output.tex` argument to the preprocessor as well. The full command
then would be:

```bash
tiefdownconverter project add-preprocessor "Enable Listings" -- -o output.tex --listings
```



The manifest would look something like this:

```toml
...

[[custom_processors.preprocessors]]
name = "Enable Listings"
pandoc_args = ["-o", "output.tex", "--listings"]

[[templates]]
filters = ["luafilters/chapter_filter.lua"]
name = "PDF Documentation LaTeX"
output = "docs_tex.pdf"
preprocessor = "Enable Listings"
template_file = "docs.tex"
template_type = "Tex"

...
```

## Custom Pandoc Conversion

I already hinted at it in [Custom Pandoc Converter](#custom-pandoc-converter),
but I'll go into more detail here. The idea is to run a preprocessor
and just skip any further processing. Straight from pandoc to the output.

You can do this by first defining a preprocessor, for example:

```bash
tiefdownconverter project add-preprocessor "RTF Preprocessor" -- -o documentation.rtf
```


As you can see, we're outputting as an RTF file, and the file name is
`documentation.rtf`. This means we need to add a template that deals with the 
same output:

```bash
tiefdownconverter project add-template "RTF Template" -o documentation.rtf -t custompandoc
```


And that's it. TiefDownConverter will run the preprocessor, which 
outputs to documentation.rtf, and then the templating system will 
copy that output to your directory. Hopefully. Did I mention that
this is experimental? Yeah, so if you have issues, please report 
them. Even if you're thinking "this is not a bug, it's a feature". 
It likely isn't.

## Smart Cleaning

Smart cleaning is a feature that is relatively simple. If you
enable it in your manifest, it will automatically remove stale or
old conversion directories.

Enable it with the `--smart-clean` and set the threshold with
`--smart-clean-threshold`. The threshold is 5 by default. 

You can also manually trigger a smart clean with 
`tiefdownconverter project smart-clean`  or a normal clean with
`tiefdownconverter project clean` . The latter will remove all
conversion directories, while the former will only remove the ones
that are older than the threshold.
