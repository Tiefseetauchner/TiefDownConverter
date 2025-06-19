# Markdown Projects {#markdown-projects}

A TiefDown project can contain multiple markdown projects. Each project defines
where the source files live and where the converted results should be placed.
The information is stored in `[[markdown_projects]]` entries in `manifest.toml`.

```toml
[[markdown_projects]]
name = "Book One"
path = "book_one/markdown"
output = "book_one/output"
```

A markdown project may define a `default_profile` used for conversion, a list
of `resources` to copy into the conversion folder and its own metadata.

## Custom Resources {#custom-resources}

Resources are additional files that are copied from the markdown project
directory to the conversion folder before processing. Typical examples are
images, CSS files or fonts needed by a template. Specify them in the
`resources` array:

```toml
resources = ["resources/cover.png", "resources/styles.css"]
```

## Markdown Project Metadata {#markdown-project-metadata}

Project specific metadata is stored under the `metadata_fields` table of a
markdown project. These values are merged with the `[shared_metadata]` of the
project during conversion. When keys collide, the markdown project metadata
overrides the shared metadata.
