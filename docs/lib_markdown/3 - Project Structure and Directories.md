# Project Structure & Directories

First off, the folder structure is as follows:

## TiefDown Folder Example
1
```
.
├── 2025-12-30_23-23-26
│   ├── docs.aux
│   ├── docs.log
│   ├── docs.out
│   ├── docs.pdf
│   ├── docs.synctex.gz
│   ├── docs.tex
│   ├── docs.toc
│   ├── docs.typ
│   ├── metadata.tex
│   ├── metadata.typ
│   ├── output.tex
│   ├── output.typ
│   ├── TeX Documentation_convdir
│   │   └── Chapter 1 - Introduction.md
│   ├── TeX Documentation.pdf
│   ├── Typst Documentation_convdir
│   │   └── Chapter 1 - Introduction.md
│   └── Typst Documentation.pdf
├── manifest.toml
├── Markdown
│   └── Chapter 1 - Introduction.md
├── template
│   ├── docs.tex
│   └── docs.typ
├── TeX Documentation.pdf
└── Typst Documentation.pdf
```

Basically, the files in `markdown/` are converted into the PDFs in the main directory using the templates in `template/`. `2025-12-31_04-00-45/` is the temporary conversion directory, containing the files needed for the conversion (Yes, I wrote this on new years eve at 4 a.m.).

## TiefDown Manifest Example

Since the source of truth in any TiefDown project is the manifest, let us have a *very* quick look at that. This manifest example follows the folder structure above.

```toml
version = 6

[custom_processors]
preprocessors = []
processors = []

[[markdown_projects]]
name = "Markdown"
output = "."
path = "Markdown"

[[templates]]
name = "Typst Documentation"
template_file = "docs.typ"
template_type = "Typst"

[[templates]]
name = "TeX Documentation"
template_file = "docs.tex"
template_type = "Tex"

[shared_metadata]
title = "Documentation"
```

As you can see, there's two templates defined in this project, as well as shared metadata that can get injected into the templates. No Custom Preprocessors or Processors are defined here.

## Template Directory

The `template/` directory is mandatory for Epub, Typst and TeX templates, and optional for other template types. It contains files that get copied to the conversion directory, and is thus the source of truth for shared files like templates, resources and lua filters. 

Since this directory gets copied to the conversion dir for every markdown project, resources are shared. For markdown project specific resources, see [custom resources](#custom-resources-copying).

## Markdown Project Directories

Markdown projects represent the input directories of a project. There can be multiple markdown projects. A markdown project has an input as well as output path and optionally resources. See [markdown projects](#markdown-projects) for more details.
