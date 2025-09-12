# Usage

The basic usage of `tiefdownconverter` is relatively simple.
The difficult part is understanding the templating system and
how to customise it for your usecases. Presets can only do so much.

> Note: I wrote this paragraph before the big refactor. The basic
> usage is no longer simple.

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
`tiefdownconverter convert -p docs`.

## Getting started

TL;DR: Make a folder, go into it and run `tiefdownconverter init` and
`tiefdownconverter convert`. That's it.

Long anser: First off, you need to create a project using `tiefdownconverter init`. This will
create a new project **in the current directory**. You can (and maybe should)
specify a project, like `tiefdownconverter init your_project`.

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
in this folder, make sure they're named like `... XX ... .md`, with anything following the
number being ignored. _This is important_, as the converter will use this to sort the
files for conversion, as otherwise it'd have no idea in which order they should be
converted. Essentially, the first number you include must be the number of the file in
order, so I suggest using a pattern like `Chapter X.md` or `X - ... .md`.

Now you should be able to run `tiefdownconverter convert -p path/to/your_project` (or
ommitting the -p flag if you're already in the project directory) and it should
generate a PDF file in the project directory using the default LaTeX template. 
You can now adjust the template, add your own input files (Markdown or otherwise), and so on.

## The input "directory"

Your source files are the main input for the converter, and as such their structure is
important. The converter will look for files in the `Markdown` directory (or the directory
specified during project creation) and will sort them by a chapter number. Namely, your files
should be named `Whatever X Whatever else.ext`, where X is a number (you don't have to name them 01, 02
etc., as we parse the number as an integer, leading zeros are removed). The converter will then sort them
by the first number and combine them in that order regardless of extension.

You can also add subdirectories in the input directory. These will be combined after
the file with the same number. For example, consider the following directory structure:

```
Markdown/
├── Chapter 1 - Introduction.md
├── Chapter 2 - Usage.md
├── Chapter 2 - Usage/
│   ├── Chapter 1 - Usage detail 1.md
│   └── Chapter 2 - Usage detail 2.md
└── Chapter 3 - Customisation.md
```

The converter will combine the files in the following order:

1. Chapter 1 - Introduction.md
2. Chapter 2 - Usage.md
3. Chapter 2 - Usage/Chapter 1 - Usage detail 1.md
4. Chapter 2 - Usage/Chapter 2 - Usage detail 2.md
5. Chapter 3 - Customisation.md

That is, the converter orders a directory by the same logic as other files (and even
does so recursively), and directories are combined after the file with the same number.

You can change what directory the converter looks for markdown files in by changing the
`markdown_dir` field in the manifest.toml file or saying `-m path/to/markdown/dir` when
initialising the project.

## Markdown projects

Now, above is a simplified explanation. If you want the full picture, read on.

With version 0.8.0 and above, the converter can handle multiple markdown folders at the
same time. This is called a "markdown project" and it is the most convoluted way to think
about markdown directories. Basically, a TiefDown project can have multiple markdown
projects, that are loaded as described above. But they have additional information stored
in them, importantly **markdown project specific metadata**.

Now, why does this exist? Well, the basic idea is that you can have multiple projects per
project. Markdown projects per TiefDown project, that is. It's useful for books for example,
where you may have shared templates and metadata (like an author) but seperate content and
metadata (like a title) for the different books. This, in theory, simplifies the workflow
substantially - but makes it more complicated to understand.

First off, the setup. You can run

```bash
tiefdownconverter project markdown add <PROJECT_NAME> <PATH_TO_MARKDOWN_DIR> <PATH_TO_OUTPUT_DIR>
```

to add a markdown project to a TiefDown project. Per default, this is either not set at all,
using the default markdown directory and output directory, or it is set to the default
markdown directory and output directory of the TiefDown project, which are `Markdown` and
`.` respectively. Importantly, the output directory is relevant for the conversion - it is
used to seperate the templating for the different projects, as well as the markdown files.
So don't use the same output directory for multiple projects unless you hacked TDC to change
the output format to include the template name, in which case, tell me how you did it.

The output directory is also important as the templates are all saved to the same file name per
default (as in, the template output file name), and if you didn't use a different output
directory, you'd overwrite the generated output for the other project. (Unless, as I said,
you found a workaround that doesn't involve a PR.)

Project specific metadata is interesting as well, as in the end, it is merged with the shared metadata.
So when you run the conversion, first the shared metadata is loaded and then the markdown
project specific metadata, overwriting the shared metadata.

Setting metadata is done by using the meta command, similarly to the [shared-meta](#shared-metadata)
command, except that you have to specify the markdown project name as well. As an example, you may
run

```bash
tiefdownconverter project markdown meta <PROJECT_NAME> set <KEY> <VALUE>
```

to set a metadata value for a markdown project.

You can also assign resources, which are files that are copied to the compile directory from
the markdown project directory *and are ignored during the conversion process*. 
This is done by using the resource command. For example,
if you had multiple books as seperate markdown projects, you could have a `cover.png` file
for each book seperately and then use the resource management to copy it to be able to be
used in a template, for example an epub or as the cover of a PDF. Check out the
[resources command](#projectmarkdownresources) for more information.

You can also assign a [profile](#conversion-profiles) to a markdown project which, if I may
say so myself as the person who needed it, is awesome.

Imagine... Well, don't imagine. Look at this documentation on github. You can see, there's
a markdown project called `cli_markdown` and a markdown project called `man_markdown`. They both
contain relatively similar but different markdown files but importantly, they act quite different. 
One generates the manpage, and one the documentation you are reading right now. These are two
completely different tasks, so the `man_markdown` project uses a differen profile per default.

A default profile is assigned using the `--default-profile` flag. This is the profile that
will be used to convert the markdown project _by default_. That doesn't mean you can't use
all templates as you wish, you can always use the `--profile` flag to specify a different
profile or the `--templates` flag to specify a different set of templates.

## Input Processing

Input processing converts source files into a format your template includes. This happens
via preprocessors. Input files are grouped by extension, and each group is processed in one
shot by the matching preprocessor (default or custom). The converter concatenates the
stdout of these runs and writes it to the configured combined output file.

By default, LaTeX templates use `output.tex` and Typst templates use `output.typ`. When you
assign preprocessors to a template, you also specify the combined output filename.

Preprocessors can be extension-specific. See [Preprocessing](#preprocessing) for details.

## Customising the template

The key idea behind tiefdownconverter is that it can handle multiple templates at the
same time. This is done by creating a template file in the template directory and adding
it to the project's manifest.toml file.

You could do this manually, if you were so inclined, but using
`tiefdownconverter project template <TEMPLATE_NAME> add` is much easier. Check the
[Usage Details](#usage-details) and specifically [the templates add command](#projecttemplatesadd)
for the usage of this command. But importantly, once you
created the template and added it to the manifest, you will be able to convert using it.
`tiefdownconverter convert -p path/to/your_project --templates <TEMPLATE_NAME>` will convert
only the selected template, aiding in debugging.

And now, you're pretty much free to do whatever you want with the template. Write tex or typst
templates, use custom filters, so on.

## Adjusting template behaviour

You have a few options for editing template behaviour using `tiefdownconverter`. You can of
course edit the template files directly, but there are a few more options.

Mainly and most interestingly, lua filters can adjust the behaviour of the markdown conversion.
These are lua scripts that are run by pandoc before the markdown is converted to tex or typst. 
You can add lua filters to a template by either editing the manifest or using
`tiefdownconverter project templates <TEMPLATE_NAME> update --add-filters <FILTER_NAME>`. This
can be either the path to a lua filter (relative to the project directory) or a directory
containing lua filters. Look up [Lua Filters](#using-lua-filters) for more information

You can also change the name of the exported file by setting the `output` option. For example,
`tiefdownconverter project templates <TEMPLATE_NAME> update --output <NEW_NAME>`. This will
export the template to `<NEW_NAME>` instead of the default `<TEMPLATE_NAME>.pdf`. This field
is required where the output extension isn't knowable, so for Custom Preprocessors/Processor
conversions.

Similarly, you could change the template file and type, though I advice against it, as this
may break the template. I advice to just add a new template and remove the old one using
`tiefdownconverter project templates <TEMPLATE_NAME> remove`.

## Conversion Profiles

A conversion profile is a shortcut to defining templates for the conversion. If you're dealing with
a lot of templates, you may be considering only converting some at any time - for example, web ready
PDFs vs. print ready PDFs, or only converting a certain size of PDF.

For that, there are conversion profiles which simply are a list of templates. It's essentially like
saving your --templates arguments.

You can create these profiles with the `project profile add` command, setting a name and a comma
seperated list of templates. Removing a profile is also possible with the `project profile remove`
command.

Running a conversion with a profile is as simple as adding the `--profile` flag.

The manifest file contains a section for this, if you desire to configure them
manually:

```toml
[[profiles]]
name = "PDF"
templates = ["PDF Documentation LaTeX", "PDF Documentation"]
```

Conversion profiles can also be set as a default for a Markdown Project, which will by default only convert
the templates in the profile when converting that project. That means, when running TDC without a
`--templates` or `--profile` argument, it will use the templates in the assigned profile only.

## Writing templates

Importantly, when you write your own template, you need to include the content somehow.
That somehow is done via `\input{output.tex}` or `#include "./output.typ"`. This will include the
output of the Markdown conversion in your template file. If you're using custom preprocessors, you
can change the output file of the conversion. See [Preprocessing](#preprocessing) for more
information. For CustomPreprocessors conversion, this is the output file already, as there is no
template file. Should you be using CustomProcessor conversion, the combined file is AST and not
really usable, so don't think about it. See [custom processor conversion](#custom-processor-converter)
for more information.

## Shared Metadata {#shared-metadata}

Metadata is a key part of any project. That's why TiefDown allows adding project wide metadata
as well as per markdown project metadata (see [Markdown Projects](#markdown-projects)). This
makes sharing metadata not only between templates easier, but even between projects.

Imagine you want to manage four books. Each of them has a different author, but the same
publisher. You could add the publisher to the metadata of each project, but that would be a lot of
work, especially if the publisher changes branding. Instead, you can add the publisher to the
shared metadata, and then add the author to the metadata of each project.

To add metadata to a project, use the `tiefdownconverter project shared-meta set` command. This
writes the metadata to the project's manifest.toml file and when converting the project, the
metadata will be written to respective metadata files for the template type (e.g. metadata.tex
for LaTeX and metadata.typ for Typst) or be used to replace arguments during the conversion. 
You can then import these files in your template and access the metadata.

### Accessing metadata in LaTeX

Metadata, per default, is accessed in LaTeX via the `\meta` command. This command takes a key
and returns the value of that key. However, accessing undefined values is undefined behaviour.
Be careful to only access metadata that is defined, and double check, I never know what happens
when you access undefined metadata. It may just throw an error, it may also write random
characters to your document.

### Accessing metadata in Typst

Much nicer than LaTeX, Typst has a type system! Just import `meta` and access the keys on it.
Though the linter will error out if you do this, so you can write your own `metadata.typ` in
the template directory with placeholder values.

### Using metadata for custom preprocessors and processors

Now, I mentioned argument replacement. Custom preprocessors or processors may include
arguments like `{{title}}`, which, during conversion, are replaced with the metadata field
`title` if available. That means that you can, for example, use `--title {{title}}` as an
argument to a custom processor for a CustomProcessor template that converts to HTML to set
the title field of said HTML file. It's more complicated, but if you know Pandoc, you know
what I mean (I hope).

## Epub Support

EPUB support in TiefDownConverter isn’t as fancy as LaTeX or Typst, but you can still tweak it to
look nice. You don’t get full-blown templates, but you can mess with CSS, fonts, and Lua filters
to make it work how you want. _This template type is however somewhat depricated. It will not be
removed but there likely won't be any new features added to it._

### Customizing CSS

EPUBs use stylesheets to control how everything looks. Any `.css` file you drop into
`template/my_epub_template/` gets automatically loaded.

For example, you can change the font, line height, and margins like so:

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

Fonts go into `template/my_epub_template/fonts/`, and TiefDownConverter will
automatically pick them up. To use them, you just need to reference them
properly in your CSS:

```css
@font-face {
  font-family: "EB Garamond";
  font-style: normal;
  font-weight: normal;
  src: url("../fonts/EBGaramond-Regular.ttf");
}

body {
  font-family: "EB Garamond", serif;
}
```

This is a good time to mention, epub is just a zip file. As such, as it is generated by
pandoc, it has a predefined structure, and you have to bend to that. Fonts are in a
font directory, and stylesheets in a styles directory. Thus you have to _break out_ of
the styles directory with .. to get to the fonts directory. Keep that in mind, it took
me a while to figure out.

### Metadata and Structure

EPUBs need some basic metadata, which you define in the YAML front matter of your Markdown
files. Stuff like title, author, and language goes here:

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

You can also do this via the custom processor arguments, adding metadata as described
in the pandoc documentation. For example, to use a seperate metadata file, you can do this:

```bash
tiefdownconverter project [PROJECT_NAME] processors add "Metadata for EPUB" -- --metadata-file metadata.yaml
tiefdownconverter project [PROJECT_NAME] templates <TEMPLATE_NAME> update --processor "Metadata for EPUB"
```

This will include the metadata file in the conversion process, removing the
need for the YAML front matter in your Markdown files and allowing you to use
different metadata files for different templates.

### Using Lua Filters

Want to tweak the structure? That’s what Lua filters are for. You can use them to rename chapters, remove junk, or modify how elements are processed.

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

There are currently five ways to convert your files. All of them are based on the same
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
Lua filters.

However, you can still get some customization by including CSS and font files in your template folder. That's
the reason epub has to have a folder in the first place, so you can place CSS and font files in there.
Of course you can add multiple epub templates, but I don't know why you would want to.

EPUB output is particularly useful for digital publishing, ensuring compatibility with e-readers
and mobile devices.

### Custom Preprocessors Converter

Okay. Stick with me here. The idea is, you are already converting my input files with Pandoc, why not let
me convert them to whatever format? Well, this is where Custom Preprocessors Conversion comes in. This long
fabled feature is the most complicated one, and you need a deep understanding of how TiefDownConverter works
and at least the ability to read Pandoc's documentation to even use it.
But if you're willing to put in the effort, you can do some pretty cool things.

The basic idea is, just, let the user decide what pandoc does. The result is chaos.

I'm being facetious, but this is actually the most powerful way to customize the
output. You add one or multiple preprocessors as described in [Preprocessing](#preprocessing) and
set the output path of the preprocessor and template to the same path. Then you can
do whatever pandoc allows. Want to convert to RTF? No issue. But beware:
you need to actually understand what's going on, otherwise you'll end up in
implementation hell.

One important thing to keep in mind is to never, ever, try to generate a non-concatenateable format
(like docx, pdf, ...) with CustomPreprocessors conversion. It won't work as soon as you have more than
one input format. Use [custom processor conversion](#custom-processor-converter) instead. 

### Custom Processor Converter

If that wasn't bad enough: We've got more. Custom Processor Conversions are a way to combine multiple
input files to a file type that isn't just a collection of lines. For example, take a docx file. It
isn't just multiple simpler files strung together, it is a complicated web of zip files and openxml
and all that jazz.

Now, why may that be an issue? Simply put: multiple input formats are converted in batches and strung
together when using a custom preprocessor converter. Instead, we need to convert all output formats
to a common type and merge them afterwards for the conversion to a docx to work.

That's what custom processor conversion is for. It uses preprocessors to convert all input files
to a common format (Pandoc native AST) and combines these files to arrive at a single AST file,
letting pandoc then convert to the required format using a custom processor.

Custom processors require an output file compatible with pandoc. When creating such a template,
make sure to reference the pandoc guide. You can use custom preprocessors as usual, but you will
need to set the output format flag (`-t`) to `native`. A custom processor can also be used when
converting from AST to the output format. Any pandoc parameter is accepted, but the `-o` and
`-f` flags are set at compile time and mustn't be added.

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
to the intermediate output file (in case of LaTeX, this is `output.tex`). But if you, and this
is a purely hypothetical scenareo, want to run a conversion on mixed input md and typ files,
you can define a typst specific preprocessor that simply uses cat.

If you want to define a preprocessor, you can do so by running

```bash
tiefdownconverter project templates <TEMPLATE_NAME> update \
  --preprocessors <PREPROCESSOR_NAMES,...> \
  --preprocessor-output <PREPROCESSOR_OUTPUT>
```

to assign it to a template and

```bash
tiefdownconverter project pre-processors add <PREPROCESSOR_NAME> -- [CLI_ARGS]
```

to create a new preprocessor.

For example, if you want to add `--listings` to the pandoc command, you could do so by adding
`--listings` to the preprocessor. But importantly, **this overwrites the default preprocessor**.
Defaults from that (as few as they may be) won't get carried over to the conversion.

```bash
tiefdownconverter project pre-processors add "Enable Listings" -- --listings
```

The manifest would look something like this:

```toml
...

[[custom_processors.preprocessors]]
name = "Enable Listings"
cli_args = ["--listings"]

[[templates]]
filters = ["luafilters/chapter_filter.lua"]
name = "PDF Documentation LaTeX"
output = "docs_tex.pdf"
template_file = "docs.tex"
template_type = "Tex"

[templates.preprocessors]
preprocessors = ["Enable Listings"]
combined_output = "output.tex"

...
```

Now, you may be able to spot a neato featureo: preprocessors are assigned in an array.
That means, you can have multiple preprocessors per template. With this power however
comes the responsibility to define extension filters on your preprocessors. This is an
extension-only glob pattern set via the `--filter` option when creating the preprocessor.

```bash
tiefdownconverter project pre-processors add "No typst conversion" --filter "typ" --cli "cat"
```

If no filter is provided, the preprocessor applies to all files. In your template, you
can then define the preprocessor list as well as the combined output of the preprocessor.
This is important, as this output is then passed to the conversion engine (or copied for
[Custom Preprocessors Conversion](#custom-preprocessors-conversion)).

## Custom Preprocessors Conversion

I already hinted at it in [Custom Preprocessors Converter](#custom-preprocessors-converter),
but I'll go into more detail here. The idea is to run a preprocessor
and just skip any further processing. Straight from pandoc to the output.

You can do this by first defining a preprocessor, for example:

```bash
tiefdownconverter project pre-processors add "RTF Preprocessor" -- -t rtf
```

As you can see, we're outputting as an RTF file. This means we need to add a template
that deals with the
same output:

```bash
tiefdownconverter project template "RTF Template" add -o documentation.rtf -t custompandoc
tiefdownconverter project template "RTF Template" update --preprocessors "RTF Preprocessor" --preprocessor-output "documentation.rtf"
```

And that's it. TiefDownConverter will run the preprocessor, which
outputs to documentation.rtf, and then the templating system will
copy that output to your directory. Hopefully. Did I mention that
this is experimental? Yeah, so if you have issues, please report
them. Even if you're thinking "this is not a bug, it's a feature".
It likely isn't.

## Custom Processor Arguments

You can define custom arguments for your processors. These are
passed to the processor, so xelatex, typst, so on, on compilation.
For example, if you needed to add a font directory to your typst
conversion, you could do so by adding the following to your manifest:

```toml
...

[[custom_processors.processors]]
name = "Typst Font Directory"
processor_args = ["--font-path", "fonts/"]

[[templates]]
name = "PDF Documentation"
output = "docs.pdf"
processor = "Typst Font Directory"
template_file = "docs.typ"
template_type = "Typst"

...
```

Or... Just use the command to create it.

```bash
tiefdownconverter project processor "Typst Font Directory" add -- --font-path fonts/
```

Then append it to a template.

```bash
tiefdownconverter project template "PDF Documentation" update --processor "Typst Font Directory"
```

This is especially useful with [custom processor converters](#custom-processor-converter).

## Smart Cleaning

Smart cleaning is a feature that is relatively simple. If you
enable it in your manifest, it will automatically remove stale or
old conversion directories.

Enable it with the `--smart-clean` and set the threshold with
`--smart-clean-threshold`. The threshold is 5 by default.

You can also manually trigger a smart clean with
`tiefdownconverter project smart-clean` or a normal clean with
`tiefdownconverter project clean`. The latter will remove all
conversion directories, while the former will only remove the ones
that are older than the threshold.
