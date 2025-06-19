# Profiles {#profiles}

A profile is a named list of templates that can be converted together. Defining
profiles avoids having to pass a long list of template names every time you run
the converter.

Profiles are stored in the project's `manifest.toml`:

```toml
[[profiles]]
name = "Documentation"
templates = ["PDF Documentation LaTeX", "PDF Documentation"]
```

Use the `--profile` option with `tiefdownconverter convert` to select a profile.
Markdown projects may also specify a `default_profile`; this profile is used if
none is supplied on the command line.
