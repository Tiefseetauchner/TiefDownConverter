# TiefDown Converter

[![GitHub Release](https://img.shields.io/github/v/release/tiefseetauchner/tiefdownconverter?sort=semver&style=for-the-badge)](https://github.com/Tiefseetauchner/TiefDownConverter/releases)
[![Crates.io Version](https://img.shields.io/crates/v/tiefdownconverter?style=for-the-badge)](https://crates.io/crates/tiefdownconverter)
[![](https://dcbadge.limes.pink/api/server/https://discord.gg/EG3zU9cTFx)](https://discord.gg/EG3zU9cTFx)

## Overview

TiefDown Converter is a command-line tool designed to streamline the conversion of structured Markdown projects into various output formats, such as PDF, EPUB, and Typst-based documents. It simplifies the process by acting as a wrapper around Pandoc and XeTeX, enabling users to set up a project once and reproducibly generate multiple formats with a single command.

### Why TiefDown?

- **One-Command Workflow**: TiefDown removes the need for complex Pandoc CLI setups, automating template management and format conversions.
- **Project-Based Structure**: Every TiefDown project is self-contained, making it easy to manage large documents.
- **Multi-Format Support**: Convert Markdown into PDFs (via XeTeX), EPUBs, or other formats using Typst.
- **Extensibility**: Customize projects with templates and Lua filters for advanced document processing.

## Documentation

Documentation is available in [docs/docs.pdf](https://github.com/Tiefseetauchner/TiefDownConverter/blob/main/docs/docs.pdf) or [docs/docs_tex.pdf](https://github.com/Tiefseetauchner/TiefDownConverter/blob/main/docs/docs_tex.pdf)

These are generated using `tiefdownconverter` as well, thus the two files. It's essentially a demo.

## Features

- Converts Markdown projects into PDFs, EPUB, and Typst-based documents.
- Reads structured project metadata from `manifest.toml`.
- Supports customizable LaTeX, Typst, and EPUB templates.
- Allows users to add, remove, and update templates.
- Enables users to validate project structure and clean temporary files.
- Simple command-line interface with easy project setup and updates.

## Installation

### Prebuilt Binaries

Download the latest release from [GitHub Releases](https://github.com/Tiefseetauchner/TiefDownConverter/releases) and extract the binary.

### Build from Source

To build TiefDown Converter manually, ensure you have Rust installed, then run:

```sh
cargo build --release
```

This will create an executable in the `target/release/` directory.

### Dependencies

TiefDown requires the following external dependencies:

- **Pandoc**: Handles Markdown parsing and conversion.
- **XeTeX**: Required for LaTeX-based PDF generation.
- **Typst** (optional): Enables Typst-based conversion.

#### Installing Dependencies

##### Linux (Debian/Ubuntu-based):

```sh
sudo apt install pandoc texlive-xetex typst
```

##### Windows (via MiKTeX):

1. Install [MiKTeX](https://miktex.org/download) or via `winget install MiKTeX`.
2. Install Pandoc from [pandoc.org](https://pandoc.org/) or via `winget install pandoc`.
3. Install Typst manually if needed or via `winget install typst`.

## Usage

### Available Commands

- **Convert** – Converts a TiefDown project, using either the current directory or a specified project. Templates can be selected manually or default ones from the manifest will be used.
- **Initialize** – Creates a new TiefDown project with optional templates. Supports setting up a Markdown directory, skipping default templates, and overwriting existing projects.
- **Project Management** – Offers various subcommands to manage TiefDown projects:
  - Add, remove, and update templates.
  - Update the project manifest.
  - List available templates.
  - Validate the project structure and metadata.
  - Clean temporary files.
- **Help** – Displays help information for any command.

### Initializing a Project

```sh
mkdir my-project
cd my-project
tiefdownconverter init -t lix_novel_a4.tex # Or any preset template you may have in mind. For no initial templates, use -n
```

This creates a new TiefDown project with the `lix_novel_a4.tex` template.

### Converting a Project

```sh
tiefdownconverter convert
```

This command converts the current project using the specified template.

#### Converting specific templates

```sh
tiefdownconverter convert -t lix_novel_book.tex
```

#### Adding a Lua Filter

Lua filters can be added to a template using the `tiefdownconverter project update-template` command:

```sh
tiefdownconverter project update-template <TEMPLATE> --add-filters path/to/filter.lua
```

To remove a filter:

```sh
tiefdownconverter project update-template <TEMPLATE> --remove-filters path/to/filter.lua
```

Filters are stored in the project manifest and used during the conversion process.

## Project Structure

A typical TiefDown project consists of:

```
project/
├── manifest.toml
├── Markdown/
│   ├── Chapter 0_ Authors notes.md
│   ├── Chapter 1.md
│   ├── ...
├── template/
│   ├── lix_novel_a4.tex
│   ├── lix_novel_book.tex
│   ├── custom_template.typ
│   ├── template_epub/
```

### `manifest.toml` Example

```toml
markdown_dir = "My Story Folder"
version = 1

[[templates]]
filters = ["luafilters/chapter_filter.lua"]
name = "lix_novel_a4.tex"
output = "a4_main.pdf"
template_type = "Tex"

[[templates]]
filters = ["luafilters/chapter_filter.lua"]
name = "lix_novel_book.tex"
output = "8x5in_main.pdf"
template_type = "Tex"
```

## Customization

### Custom Templates

Users are encouraged to create their own templates. TiefDown allows adding templates via:

```sh
tiefdownconverter project add-template my_template.tex
```

Use -h for more options.

Templates must include:

```latex
\input{./output.tex}
```

to correctly insert converted Markdown content.

### Custom Lua Filters

Users can define and apply custom Lua filters by placing them in the project directory and updating the manifest as described above.

Example filter to modify headers:

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

## Performance & Limitations

- Handles large documents as efficiently as Pandoc and XeTeX allow.
- Minimal logging/debugging available.
- No Windows installer yet; setup is manual.
- EPUB output is functional but lacks advanced customization.

## Community & Support

### Contributions

Contributions are welcome via pull requests on GitHub. Please be kind.

### Bug Reports & Feature Requests

Report issues on the [GitHub Issues page](https://github.com/Tiefseetauchner/TiefDownConverter/issues).

You can also join my [Discord Server](https://discord.gg/EG3zU9cTFx)

## License

This project is licensed under MIT. See `LICENSE` for details.

## Coffee
If you appreciate this project, consider supporting it by contributing or donating.

[![](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/tiefseetauchner)
