# Manifest

The manifest is the source of truth for TiefDown. It, and only it, defines the behavior of any TiefDown conversion. Note that the examples here are non-exhaustive. They simply serve to give you an idea of what to expect.

## Versioning and compatibility/upgrades

The manifest version is at the top of the manifest, and similarly important. It consists of a single integer. Since TiefDown is under constant development, old manifests must be upgradeable in a consistent and expectable way. It also prevents an old version of TiefDownLib from messing up a manifest generated from a newer version.

The upgrade process consists of sequentially upgrading the manifest through the versions until it reaches the current version. That way, upgrades are reproducible. It also means that every prior version of the manifest is upgradeable.

Example:

```toml
version = 6
```

## Markdown Projects List

Markdown projects are the cornerstone of TiefDown. They define the input files and parameters. They can be used for a variety of use cases, but are especially useful when template sharing is a concern.

There can be multiple markdown projects per TiefDown project.

A markdown project contains, among others, an input and output directory, allowing clustering of output files. See [markdown projects](#markdown-projects) for more information.

Example:

```toml
[[markdown_projects]]
default_profile = "Man"
name = "man"
output = "man"
path = "man_markdown"
```

## Templates List

Templates are the basis for document conversion. Each template can be applied to multiple markdown projects using the [queueing system](#queueing-system).

Example:

```toml
[[templates]]
filters = ["luafilters/mega_replacer_filter.lua"]
footer_injections = ["HTML footer"]
header_injections = ["HTML header"]
name = "GitHub Single File Documentation"
output = "index.html"
template_type = "CustomPreprocessors"

[templates.preprocessors]
combined_output = "index.html"
output_extension = "html"
preprocessors = ["HTML Conversion", "HTML Direct Copy"]
```

## Custom Processors model

Custom processors and custom preprocessors are extensions on the usual conversion process, changing the arguments passed to the pandoc process, or even changing the executable of the preprocessing.

Example:

```toml
[[custom_processors.preprocessors]]
cli = "cat"
cli_args = []
extension_filter = "html"
name = "HTML Direct Copy"

[[custom_processors.processors]]
name = "HTML Standalone Conversion"
processor_args = ["--to", "html5", "-s", "--metadata", "title={{title}}", "--metadata", "author={{author}}", "--css", "/TiefDownConverter/template/html_template/style.css", "--toc", "-B", "html_template/header.html"]
```

## Shared Metadata

Metadata in TiefDown is split in two parts: shared metadata that is accessible from all markdown projects and markdown project specific metadata.

Examples:

- Shared metadata:
  ```toml
  [shared_metadata]
  author = "Tiefseetauchner et al."
  githubPagesUrl = "https://tiefseetauchner.github.io/TiefDownConverter/"
  title = "TiefDownConverter Documentation"
  ```
- Markdown project specific metadata:
  ```toml
  [markdown_projects.metadata_fields]
  githubPagesDocsPath = "/"
  ```

## Metadata Settings

Metadata settings define how metadata is injected into the conversion process.

Example:

```toml
[metadata_settings]
metadata_prefix = "projectMetadata"
```

## Profiles

Profiles allow bundling of templates into a preset execution order, allowing the creation of subgroups as well as defining a default profile for a markdown project.

Example:

```toml
[[profiles]]
name = "Documentation"
templates = ["PDF Documentation LaTeX", "PDF Documentation", "Epub Documentation", "GitHub Multi Page Documentation"]
```

## Injections

Injections are the intended way to create template specific conversion additions. There are header, body, and footer injections, allowing the user to insert template specific markup into all parts of the conversion process.

Example:

```toml
[[injections]]
files = ["head.html", "nav.md", "head2.html"]
name = "HTML header"
```

## Smart Clean Settings

Smart Clean settings control how and whether smart clean should run during conversion.

Example:

```toml
smart_clean = true
smart_clean_threshold = 3
```

## Full Example

Below is an example of a `manifest.toml` that illustrates the most important features of TiefDownConverter, including injections, preprocessors, and multiple templates.

For a detailed explanation of every part of this manifest, see below. Note that this manifest has been rearranged and indented to improve clarity.

```toml
smart_clean = true
smart_clean_threshold = 3
version = 6

[shared_metadata]
author = "Lena Tauchner"

[[markdown_projects]]
name = "Dream"
output = "Dream"
path = "Dreams/1 - Dream"
resources = ["additional_meta.tex", "epub_meta.yaml", "cover.png"]

    [markdown_projects.metadata_fields]
    edition_date = "2025"
    edition_num = "1"
    title = "Dream"

[[markdown_projects]]
name = "Reality"
output = "Reality"
path = "Dreams/2 - Reality"
resources = ["additional_meta.tex", "epub_meta.yaml", "cover.png"]

    [markdown_projects.metadata_fields]
    edition_date = "2025"
    edition_num = "1"
    title = "Reality"

[[templates]]
filters = ["luafilters/"]
name = "lix_novel_a4.tex"
output = "a4_main.pdf"
template_type = "Tex"

[[templates]]
filters = ["luafilters/"]
name = "lix_novel_book_at.tex"
output = "140x205mm_main.pdf"
template_type = "Tex"

[[templates]]
name = "Original MD"
output = "original.md"
template_type = "CustomPreprocessors"

    [templates.preprocessors]
    combined_output = "original.md"
    preprocessors = ["Original MD"]

[[templates]]
filters = ["epub_lua_filters"]
header_injections = ["EPUB Copyright Block"]
name = "EPUB Conversion"
output = "main_ebook.epub"
processor = "Metadata for EPUB"
template_file = "main_epub/"
template_type = "Epub"

[[templates]]
filters = ["luafilters/mega_replacer_filter.lua"]
multi_file_output = true
name = "HTML Helper Export"
output = "html_raw/"
template_type = "CustomPreprocessors"

    [templates.preprocessors]
    output_extension = "html"
    preprocessors = ["HTML Multi Page", "HTML Raw"]

[[custom_processors.preprocessors]]
cli_args = ["-t", "markdown"]
name = "Original MD"

[[custom_processors.preprocessors]]
cli_args = ["-t", "html5"]
name = "HTML Multi Page"

[[custom_processors.preprocessors]]
cli = "cat"
cli_args = []
extension_filter = "html"
name = "HTML Raw"

[[custom_processors.processors]]
name = "Metadata for EPUB"
processor_args = ["--metadata-file", "epub_meta.yaml"]

[[injections]]
files = ["epub_copyright_block.html"]
name = "EPUB Copyright Block"
```

Now to explain the basic structure of the `manifest.toml`, I will start by addressing the basic project settings. Important are the `smart_clean` and `smart_clean_threshold` flags. These define the smart cleaning behavior of the project. See [Smart Clean](#smart-clean) for more information. Also note the `version` field, which defines the compatibilty with TiefDown.0

After smart cleaning, we define the `shared_metadata` table, which contains metadata about the project that is shared between the items. In this case, the author.

Then, we define the markdown projects. The markdown projects in this case are Dream and Reality, each with a separate output and input folder, as well as metadata fields for filling in the LaTeX data and resources that are project specific but used in the template. In this example, we have a `cover.png`, which, during the epub conversion, is injected as the cover image.

After defining the markdown projects, we add templates. Here, we have five templates. The LiX novel templates (`lix_novel_a4.tex` and `lix_novel_book_at.tex`), a `CustomPreprocessors` conversion to a singular markdown file, an EPUB template and the HTML Helper export, which simply exports an HTML file per input file (in my case here, chapters).

Crucially, we define `[templates.preprocessors]` for the `CustomPreprocessors` conversions, which handle different output and input formats respectively.

The aforementioned preprocessors are links to the `custom_processors.preprocessors` array, which contains all preprocessors needed for conversion. Here, that is the Markdown conversion, Markdown to HTML5 and Raw copying of HTML documents. For the last case, we use the `cli` field to change the executable ran during conversion to `cat` from `pandoc`, to simply copy the file content to memory.

For EPUB conversion, we also need the processors argument, as there is metadata used. `epub_meta.yaml`, in this case, is a static template file which simply provides additional, common metadata to epub.

Lastly, the EPUB conversion requires a copyright block specifically. In this case, that is solved as an injection, which simply places a `epub_copyright_block.html` into the epub conversions header injections.
