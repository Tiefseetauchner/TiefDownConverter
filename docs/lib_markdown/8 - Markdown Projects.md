# Markdown projects

Since TiefDown operates on structured, multifile inputs stored in directories, the directory has to be declared in the manifest. The concept behind this declaration is called "markdown projects", based on the outdated assumption that input files would always be markdown (they are not).

Regardless, markdown projects are defined and then ran through during conversion. Each markdown project has a unique identifier (`name`), an input path, and output path, optional resources and default profile, as well as optional metadata.

## Input discovery & sorting rules 

Files in the input directory defined by a markdown project are sorted by their file name order. This is calculated as the first whole number in a file name. E.g. if a file is named `Chapter 23.5 - the reckoning.md`, the extracted order is 23.

Folders similarly have an order number. Files in a folder are added recursively in accordance with the same sorting mechanism. Files in a folder (e.g. `Chapter 42 - More details/Detail 3 - The Chicken.md`) are inserted after the file with the same order number.

## Custom resources copying

As markdown projects have input files that are converted using pandoc or similar during the preprocessing step, one can define resources that are only copied but not consumed by the preprocessors. Resources are copied to the conversion directory of the markdown project instead of to the `conv_dir` of the relevant markdown project, even though they initially reside in the markdown projects' input directory.\

As a concrete example: take a cover image. The cover image would be injected into a PDF. But different markdown projects (e.g. books, papers, ...) need seperate cover images. The cover image is thus added as a resource. Take the following folder structure:

```
.
├── manifest.toml
├── Book 1 Markdown
│   ├── Chapter 1 - Introduction.md
│   └── cover.jpg
├── Book 2 Markdown
│   ├── Chapter 1 - Different Introduction.md
│   └── cover.jpg
├── Book 1
│   └── BookOut.pdf
├── Book 2
│   └── BookOut.pdf
└── template
    └── book.typ
```

where `manifest.toml` contains:

```toml
[[markdown_projects]]
name = "My Book 1"
output = "Book 1"
path = "Book 1 Markdown"
resources = ["cover.jpg"]

[[markdown_projects]]
name = "My Book 2"
output = "Book 2"
path = "Book 2 Markdown"
resources = ["cover.jpg"]

```

The `cover.jpg` in this case is copied to the conversion directory, where docs.typ can consume it.

## Project metadata fields

Projects can have specified metadata fields. These override the shared metadata. This can be helpful to adjust template behavior, e.g. changing the title of the book. See [the manifest example](#full-example).
