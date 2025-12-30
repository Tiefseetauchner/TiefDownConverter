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
- The automatic [smart clean](#smart-clean) flag
  - This is a boolean flag that determines if the project should be cleaned
    automatically when a conversion is run. This is useful for projects that
    are constantly being updated, allowing a user to decide how many
    conversion folders they want to keep.
- The smart clean threshold
  - This is the number of conversion folders that are kept before the oldest
    ones are deleted.
- A list of [markdown projects](#markdown-projects)
- A list of [templates](#templates)
- A [custom processors](#custom-processors) object
- A table of [shared metadata](#shared-metadata) for all markdown projects
- A [metadata settings](#metadata-settings) object
- A list of [profiles](#profiles) available for the conversion
- A list of [injections](#injections)

## Building your own manifest

A manifest can grow to a complex web relatively quickly. Thus, in this section, I want to give a little bit of an example on building and reading manifests. *Knowledge of TOML is presupposed*

Consider the following manifest:

```toml
smart_clean = true
smart_clean_threshold = 3
version = 6

[[markdown_projects]]
name = "Documentation"
output = "."
path = "markdown"
resources = ["resources/"]

[[templates]]
name = "Documentation"
template_file = "docs.tex"
template_type = "Tex"
```

This is the most basic manifest, corresponding to a TeX project. It defines the
project meta information, as well as a template and a markdown project. It
corresponds to a basic folder structure:

```
.
├── markdown/
│   ├── resources/
│   │   └── image1.png
│   ├── 1 - Introduction.md
│   └── 2 - TiefDown.md
├── template/
│   ├── docs.tex
│   └── lib.sty
├── Documentation.pdf
└── manifest.toml
```

Let us dissect the manifest, starting with the meta information.

```toml
smart_clean = true
smart_clean_threshold = 3
version = 6

# ...
```

`smart_clean = true` sets up the automatic [smart clean](#smart-clean) feature
to be enabled.

`smart_clean_threshold = 3` sets the threshold of automatic smart cleaning to
three.

`version = 6` defines the supported featureset of this project, setting the
required TiefDownLib version.

As far as meta information goes, TiefDown is relatively lean. Let's now look
at the `[[markdown_projects]]` section, which is TiefDowns way of specifying
an input directory, as described in [markdown projects](#markdown-projects).

```toml
# ...

[[markdown_projects]]
name = "Documentation"
output = "."
path = "markdown"
resources = ["resources/"]

# ...
```

First off, we define the markdown project with a `name` parameter. This name is
primarily for logging purposes, as well as for converting just a single 
markdown project.

`output = "."` defines the folder that the result files will be copied to. Per
default, the `.` directory defines the TiefDown projects root directory.

`path = "markdown"` is the input directory in which the input files will copied
from before conversion, and thus is the primary source of truth of input files.

`resources = ["resources/"]` is important here: it specifies that the 
`resources/` folder does not include input files and should not be converted,
but is still available during the conversion process. This allows for images 
and other resources to be added on a per-markdown-project basis without
requiring logic in the template. See [custom resources](#custom-resources)
for more information.

Lastly, there's the template section:

```toml
# ...

[[templates]]
name = "Documentation"
template_file = "docs.tex"
template_type = "Tex"
```

This defines the (in this case, singular) template.


`name = "Documentation"` sets the name of the template, which serves as basic 
meta information for conversion.

`template_file = "docs.tex"` tells TiefDown, which TeX file should be 
converted. Here, docs.tex in the `template/` folder gets converted.

`template_type = "Tex"` is the template type, in our case TeX.

That is a basic manifest toml. Following is a more complex manifest, but
beware, this is likely an excessively complex usecase.

------------------ **TODO** ------------------
