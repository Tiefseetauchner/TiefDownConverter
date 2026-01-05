# LIB Documentation

This section documents the **TiefDown project format** and the library concepts
TiefDownConverter builds on (manifest, templates, markdown projects, and the
conversion pipeline).

If you’re looking for how to *use the CLI*, jump to the [CLI docs](../cli/).

## Start here

- [Welcome to TiefDown](<documentation/1 - Introduction.html>) — what a TiefDown project is and why it exists.
- [Project Structure & Directories](<documentation/3 - Project Structure and Directories.html>) — folder layout, template dir, and scratch dirs.
- [Manifest](<documentation/4 - Manifest.html>) — the `manifest.toml` schema, versioning, and examples.

## Topics (by chapter)

- [Project Model Overview](<documentation/2 - Project Model Overview.html>) — how the pieces fit together.
- [Conversion Pipeline](<documentation/5 - Conversion Pipeline.html>) — queueing, copying, metadata merge, and converters.
- [Templates](<documentation/6 - Templates.html>) — TeX/Typst/EPUB vs custom converter templates.
- [Lua Filters](<documentation/7 - Lua Filters.html>) — discovery and where filters run.
- [Markdown Projects](<documentation/8 - Markdown Projects.html>) — input discovery/sorting, resources, and per-project metadata.
- [Injections](<documentation/9 - Injections.html>) — header/body/footer injections and when they apply.
- [Multi-file Output Model](<documentation/10 - Multi-file Output Model.html>) — one output per input file (CustomPreprocessors).
- [Metadata Generation and Injection](<documentation/11 - Metadata Generation and Injection.html>) — project vs navigation metadata and how to access it.
- [Smart Clean](<documentation/12 - Smart Clean.html>) — pruning old conversion folders automatically.
