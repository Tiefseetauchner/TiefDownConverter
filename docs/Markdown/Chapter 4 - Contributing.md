# Contributing

This project is open source, and I'd love for you to contribute!
There's a few things you should know before you start.

## Pull Requests

Pull Requests should be made with either a link to an issue or an explanation of

1. What was the problem
2. How is it solved now
3. How did it affect the documentation

It takes a lot of work to understand the intention of code you didn't write and
then judging whether this was indeed the intended outcome. That's why it's helpful
for everyone if there's an explanation on what was changed and why.

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

Currently primarily integration tests. See the `tests` folder for examples. Any pull
request to main will automatically run tests, and the expectation is that at least the
existing tests work. If they break, fix your code or, if you changed behavior on purpose, 
the tests.

I appreciate it if you add test coverage for your changes. I especially would
appreciate more unit tests, but the tests I have are sufficient for now.
Integration tests take priority over unit tests for me, as the overall behavior
is more important to me than the individual functions, and I only have so much
time that I want to spend on this project.

## Documentation

When changing the documentation, it is of utmost importance that the documentation outputs
are correctly generated. *These are not automatically generated on release* but rather held 
in git to more easily track changes during a pull request.

To make sure this documentation is up to date, consider whether your changes significantly
affect the workflow of using TiefDownConverter. If you add a command or flag, make sure to
run `tools/generate_docs.py`. Either way, when changing the documentation, always run 
`tiefdownconverter convert -p docs` before committing the changes.

**You need to have the fonts EB Garamond, and Iosevka installed!** 
If you don't, we cannot accept documentation changes. TiefDownConverter will 
throw a warning should the fonts not be installed. **The warning about Fira
Mono missing is normal. This is the fallback font and not a requirement
to compile the documentation**



## Code Style

I don't have one. I'm sorry.
