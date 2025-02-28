# TiefDown Converter

## Overview
TiefDown Converter is a command-line tool designed to convert TiefDown projects, a structured format for compiling Markdown files into PDFs. It automates the process of combining multiple Markdown files, applying LaTeX templates, and generating a PDF using XeTeX. 

### Example Use Case
Imagine you are writing a book with multiple chapters stored as separate Markdown files. With TiefDown Converter, you can structure your project, define your LaTeX template, and generate a professionally formatted PDF with just one command.

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

The `manifest.toml` is the primary indicator that this is a TiefDown project. Currently, an example configuration is:
```toml
templates = ["template.tex", "template_a4.tex"]
markdown_dir = "Markdown"
```

The `Markdown/` folder should contain all Markdown files. They should begin with "Chapter XX" to allow for sorting.

The templates can be any `.tex` file, but to include the generated content, they must include `\input{./output.tex}`. An example using [LiX's novel document class](https://github.com/NicklasVraa/LiX/) could look like this:

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

Where `meta.tex` is also in the template directory and might contain:

```latex
\lang      {english}
\title     {My Story}
\subtitle  {Small stories for bedtime 5}
\authors   {Tiefseetauchner}
\publisher {Tiefseetauchner}
\edition   {1}{2025} % Update for new editions
\keywords  {fiction}

\note{This is a work of fiction. Any resemblance to real-world persons, living or dead, is purely coincidental.}

\license{CC}{by-nc-nd}{4.0} % Modify or remove as needed
```

A Lua filter in `luafilters/` might look like this:

```lua
function Header(elem)
  if elem.level == 1 then
    return pandoc.RawBlock("latex", "\\h{" .. pandoc.utils.stringify(elem.content) .. "}")
  end
  if elem.level == 2 then
    return pandoc.RawBlock("latex", "\\hh{" .. pandoc.utils.stringify(elem.content) .. "}")
  end
end
```

## License
This project is licensed under MIT. See the license file for details.

## Coffee
If you appreciate this project, consider supporting it by contributing or donating.

[![](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/tiefseetauchner)
