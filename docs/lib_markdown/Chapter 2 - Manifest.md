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
