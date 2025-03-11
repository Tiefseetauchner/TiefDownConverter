# Contributing

This project is open source, and I'd love for you to contribute!
There's a few things you should know before you start.

## Conversion

Conversion is split in a few different steps:

1. Combine all the markdown files into one megafile called `combined.md`.
2. Run Pandoc conversion to TeX, EPUB, or Typst. This uses Lua filters that are
   defined in the `manifest.toml` file.
3. Run XeLaTeX on all TeX templates, Typst on all Typst templates, and so on.

Say you were to add a new conversion type. In `converters.rs`, you'd need to
add a new function that handles the full conversion. Including handling lua filters,
markdown conversion, so on. This converter function has to then be included in our
conversion decision logic in `conversion_decider.rs`. And for that you need to add
a new TemplateType, which includes editing the implementations. Then you need to
add the new template type decision logic to `get_template_type_from_path`.

## Presets

> **NOTE:** This is a bit of a niche usecase, so documentation is lacking. You can
> always ask for help on this in a GitHub issue.

You can also add new presets, but that's a bit more involved. You should check
the implementation for the existing presets, I don't think it's useful to document
this nieche usecase for now.

## Manifest

Hope you don't have to change the manifest.toml file.
If you do, change the manifest model, increase the version number in `consts.rs` and
add a upgrade logic to `manifest_model.rs`.

## Tests

Practically non-existant but I would be happy if you added some.

## Documentation

Make sure this documentation is up to date. If you add a command or flag, make sure to
run `tools/generate_docs.py` as well as `tiefdownconverter convert -p docs`. Then commit
the changes.

## Code Style

I don't have one. I'm sorry.
