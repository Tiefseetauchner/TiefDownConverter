# Manifest File

The manifest file is the heart of the project. It contains all the information
needed to manage and convert the project.

It consists of a few important parts (for the full documentation, check
[https://docs.rs/tiefdownlib/latest/tiefdownlib/manifest_model/index.html](https://docs.rs/tiefdownlib/latest/tiefdownlib/manifest_model/index.html)):

- A version number
  - This is used to determine if the manifest is compatible with the current
    version of TiefDownConverter. If it's not, the manifest will be
    automatically updated in the process of loading. Newer versions of the
    manifest file are rejected by the implementation.
- The automatic smart clean flag
  - This is a boolean flag that determines if the project should be cleaned
    automatically when a conversion is run. This is useful for projects that
    are constantly being updated, allowing a user to decide how many
    [conversion folders](#conversion-folders) they want to keep.
- The smart clean threshold
  - This is the number of conversion folders that are kept before the oldest
    ones are deleted.
- A list of [markdown projects](#markdown-projects)
- A list of [templates](#templates)
- A [custom processors](#custom-processors) object
- A table of [shared metadata](#shared-metadata) for all markdown projects
- A [metadata settings](#metadata-settings) object
- A list of [profiles](#profiles) available for the conversion

## Templates and processors (at a glance)

Each entry under `[[templates]]` specifies one output variant. Important fields are:

- `template_type`: `Tex`, `Typst`, `Epub`, `CustomPreprocessors`, or `CustomProcessor`.
- `template_file`: Path relative to the template directory; if omitted, the template name is used.
- `output`: Optional output filename (defaults based on template type).
- `filters`: Lua filters (file or directory paths) applied during Pandoc steps.
- `preprocessors`: Names of preprocessors plus a required `combined_output` filename the
  template includes.
- `processor`: Name of a processor whose arguments are passed to XeLaTeX/Typst.
  For `CustomProcessor`, the `processor` arguments are passed to Pandoc as the
  final conversion step.

Project-wide `[[custom_processors.preprocessors]]` and `[[custom_processors.processors]]` define
reusable building blocks referenced by templates.

Example snippets

```toml
# CustomPreprocessors: you decide how inputs are preprocessed; the combined
# output is copied to the final destination without an additional processor step.
[[templates]]
name = "Website HTML"
template_type = "CustomPreprocessors"
output = "site/index.html"

  [templates.preprocessors]
  preprocessors = ["html-from-md"]
  combined_output = "output.html"

[[custom_processors.preprocessors]]
name = "html-from-md"
cli = "pandoc"
cli_args = ["-f", "markdown", "-t", "html"]

# CustomProcessor: preprocess to Pandoc Native, then run Pandoc once more with
# processor arguments to produce the final artifact.
[[templates]]
name = "Docx Export"
template_type = "CustomProcessor"
output = "book.docx"
processor = "docx-out"

  [templates.preprocessors]
  preprocessors = ["native-parts"]
  combined_output = "output.pandoc_native"

[[custom_processors.preprocessors]]
name = "native-parts"
cli = "pandoc"
cli_args = ["-t", "native"]

[[custom_processors.processors]]
name = "docx-out"
processor_args = ["--reference-doc", "resources/reference.docx"]
```
