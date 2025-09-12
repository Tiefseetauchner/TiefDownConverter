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
cli_args = ["-t", "latex", "--listings"]
```

A preprocessor can also define a command using the `cli` field. This replaces the
Pandoc preprocessing step with a custom cli command preprocessing step.

```toml
[[custom_processors.preprocessors]]
name = "Copy without modification"
cli = "cat"
cli_args = []
extension_filter = "typ"
```

Templates reference one or more preprocessors with their `preprocessors` field, which
also has to define a `combined_output` field. The converter captures the stdout of each
preprocessor run and writes it to this file, which your template then includes
(`\input{./output.tex}` or `#include "./output.typ"`) or copies to the output
location for `CustomPreprocessors` templates.

Processors are specified similarly and referenced via the `processor` field:

```toml
[[custom_processors.processors]]
name = "Typst Font Directory"
processor_args = ["--font-path", "fonts/"]
```

Usage notes and examples

- For `CustomPreprocessors` templates, there is no processor step. You are
  responsible for ensuring the `combined_output` is the final artifact you want
  to copy to the project’s output.
- For `CustomProcessor` templates, a processor is required. TiefDown combines
  inputs to Pandoc Native (defaults provided) and then runs Pandoc with your
  `processor_args` to produce the final artifact.

Example: Reveal.js slide deck via CustomProcessor

```toml
[[templates]]
name = "Slides"
template_type = "CustomProcessor"
output = "talk.html"
processor = "reveal"

  [templates.preprocessors]
  preprocessors = ["native"]
  combined_output = "output.pandoc_native"

[[custom_processors.preprocessors]]
name = "native"
cli = "pandoc"
cli_args = ["-t", "native"]

[[custom_processors.processors]]
name = "reveal"
processor_args = ["-t", "revealjs", "-s", "--slide-level", "2"]
```

These mechanisms allow fine-grained control over the conversion pipeline when the
defaults are not sufficient.

## Defaults

TiefDown provides reasonable defaults per template type:

- Tex templates preprocess inputs with Pandoc using `-t latex`, writing
  `output.tex`.
- Typst templates use two preprocessors: Pandoc with `-t typst` for non-`.typ`
  inputs, and a pass-through step for `.typ` inputs so existing Typst files are
  concatenated. The combined output is `output.typ`.

You can override a default for a particular extension by defining a preprocessor
with a matching `extension_filter`; defaults for other extensions remain.

Preprocessors can be scoped by extension via `extension_filter`, which matches only the
file extension (glob patterns such as `t*` are supported). If you omit the filter, the
preprocessor acts as a fallback when no more specific filter matches. Defaults exist per
template type and are merged by extension; defining your own preprocessor for a particular
extension replaces the default for that extension but leaves the others intact. Finally,
`cli_args` support metadata substitution, so any occurrence of `{{key}}` is replaced with
the corresponding metadata value at conversion time.
