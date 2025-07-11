# Custom Processors {#custom-processors}

Custom processors let you change the commands used during conversion. They come
in two forms:

- **Preprocessors** replace the default pandoc invocation that generates the
  intermediate file.
- **Processors** provide additional arguments to the program that handles the
  template itself (for example XeLaTeX or Typst).

A preprocessor is defined under `[[custom_processors.preprocessors]]`:

```toml
[[custom_processors.preprocessors]]
name = "Enable Listings"
pandoc_args = ["-t", "latex", "--listings"]
combined_output = "output.tex"
```

Templates reference it with their `preprocessor` field. Processors are specified
similarly and referenced via the `processor` field:

```toml
[[custom_processors.processors]]
name = "Typst Font Directory"
processor_args = ["--font-path", "fonts/"]
```

These mechanisms allow fine-grained control over the conversion pipeline when the
defaults are not sufficient.
