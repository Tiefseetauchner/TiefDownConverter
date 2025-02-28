# TiefDown Converter

## Overview
TiefDown Converter is a command-line tool designed to convert TiefDown projects, a structured format for compiling Markdown files into PDFs. The conversion process involves copying a defined template structure, merging Markdown files, converting the combined file to LaTeX, and generating a PDF using XeTeX.

## Features
- Reads project structure from `manifest.toml`
- Copies template files to a build directory
- Merges Markdown files into a single document
- Converts Markdown to LaTeX and compiles to PDF
- Uses Lua filters for customization
- Simple command-line interface

## Requirements
- [Rust (for compiling the tool)]
- Pandoc (for Markdown to LaTeX conversion)
- XeTeX (for compiling LaTeX to PDF)

## Installation
To build the project from source, run:
```sh
cargo build --release
```
This will create an executable in the `target/release/` directory.

## Usage
Run the converter with:
```sh
./tiefdownconverter convert [-p project_directory]
```
Additional commands for project management will be added in future updates.

## Project Structure
A typical TiefDown project may look like this:
```
project/
├── manifest.toml
├── Markdown/
│   ├── Chapter 0: Authors notes
│   ├── Chapter 1: Nicoletta
│   ├── ...
├── template/
│   ├── template.tex
│   ├── template_a4.tex
├── luafilters/
│   ├── chapter_filter.lua
│   ├── other_filter.lua
```

The `manifest.toml` is the primary indicator that this is a tiefdown project. Currently, my example is
```toml
templates = ["template.tex", "template_a4.tex"]
```

The folder `Markdown/` should contain all Markdown files. They should begin with "Chapter XX" to allow for sorting.

The templates can be any tex file, but to include the generated content, they have to include `\input{./output.tex}`. An example using [LiX's novel document class](https://github.com/NicklasVraa/LiX/) could look like this:

```latex
\documentclass{novel}

\input{./meta.tex}

\size{custom}{5.5in}{8.5in}
\margins{20mm}{25mm}{25mm}{20mm}

\begin{document}

\pagenumbering{}
\setcounter{page}{1}

\hspace*{10pt}
\clearpage

\tableofcontents

\cleardoublepage

\input{./output.tex}

\end{document}
```

Where meta.tex is also in the template directory. It would contain something like this:

```latex
\lang      {english}
\title     {My Story}
\subtitle  {Small stories for bedtime 5}
\authors   {Tiefseetauchner}
\publisher {Tiefseetauchner}
\edition   {1}{2025} % you should edit this only when creating a new edition. For example, if you found a significant mistake, you may change the edition to two and the year to the current year.
\keywords  {fiction}

\note{This is a work of fiction. Any resemblance to real-world persons, living or dead, per name or otherwise, is purely coincidental. \\
Beyond the Past © 2025 by Lena Tauchner is licensed under CC BY-NC-ND 4.0, as described by the license section.}

\license{CC}{by-nc-nd}{4.0} % you may want to remove the license. If so, don't forget to remove the above disclaimer!
```

This also requires a lua filter in luafilters/ like this:

```lua
function Header(elem)
  -- Check if it's a first-level heading (i.e., Chapter in Markdown)
  if elem.level == 1 then
    -- Replace with \h{...} LaTeX command
    return pandoc.RawBlock("latex", "\\h{" .. pandoc.utils.stringify(elem.content) .. "}")
  end
  if elem.level == 2 then
    -- Replace with \hh{...} LaTeX command
    return pandoc.RawBlock("latex", "\\hh{" .. pandoc.utils.stringify(elem.content) .. "}")
  end
end
```

## License
This project is licensed under MIT. See the license file for information.

## Coffee
You know it by now. I am doing a lot of projects for fun, and I appreciate any help you can give - be that by contributing or donating.

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/tiefseetauchner)

