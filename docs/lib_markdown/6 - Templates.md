# Templates

A pivotal point in any TiefDown user's life is once they truly understand templating. This section seeks to serve that purpose.

Templating in TiefDown is separated into two archetypes: template-based templates and logic-based templates.

TeX, Typst, and EPUB templates are template-based. They rely on a concrete template file located in the template directory, which defines the structure of the final output and into which converted content and metadata are injected.

CustomPreprocessors and CustomProcessor templates are logic-based. They define the conversion process purely through configured commands and processors, without relying on a single template file. They may still make use of files from the template directory, but the execution flow is driven by logic rather than a fixed document template.

For execution logic of each of the templates, see [Running Conversion](#running-conversion) from the previous chapter.


## TeX templates

TeX templates use a LaTeX file as the template for the primary execution point. This template file must be specified in the template, and should `\input` the combined output from the pandoc conversion.

Metadata is written separately to a `metadata.tex` file, which creates macros for accessing metadata. Below is an example of such a `metadata.tex` file:

```tex
\newcommand{\meta}[1]{\csname meta@#1\endcsname}

\expandafter\def\csname meta@author\endcsname{Tiefseetauchner et al.}
\expandafter\def\csname meta@githubPagesDocsPath\endcsname{lib/}
\expandafter\def\csname meta@githubPagesUrl\endcsname{https://tiefseetauchner.github.io/TiefDownConverter/}
\expandafter\def\csname meta@title\endcsname{TiefDownConverter Documentation}
```

Metadata can then be accessed in the template file or in `.tex` files in the input like so:

```tex
\input{./metadata.tex}
\meta{title}
% Writes "TiefDownConverter Documentation"
```

Lua filters and preprocessors in LaTeX are fully supported. For example:

- TeX Raw\
  A `cat` preprocessor that copies `.tex` files instead of running them through pandoc.
- --listings\
  Pandoc supports outputting LaTeX with listings support. A preprocessor can enable that for your file.

Processor arguments are fully supported. Processor arguments can be used to adjust the XeTeX cli call.

## Typst templates

Similarly to TeX templates, Typst templates also use a template file, in this case a `.typ` file, for conversion. This template file must be specified in the template, and should `#include` the combined output from the pandoc conversion.

Metadata is also written to a `metadata.typ` file, which creates a dictionary for accessing metadata. Below is the same example as above for Typst:

```typst
#let meta = (
  author: "Tiefseetauchner et al.",
  githubPagesDocsPath: "lib/",
  githubPagesUrl: "https://tiefseetauchner.github.io/TiefDownConverter/",
  title: "TiefDownConverter Documentation",
)
```

Accessing metadata then becomes trivial:

```typst
#import "metadata.typ": meta
#meta.title
// Writes "TiefDownConverter Documentation
```

Equally to the above template, lua filters and preprocessors are fully supported for Typst.

Processor arguments, as above, are fully supported. The processor arguments are added to the typst process on conversion.

## EPUB templates

EPUB templates are a special kind of template, as they are less user-unfriendly than CustomProcessor converters.

The primary simplification in EPUB templates is the addition of css and font search. For conversion, EPUB retrieves CSS as well as font files from the template directory and injects them into the output file. Fonts are searched within a `fonts/` subfolder in the template.

Lua filters are fully supported for epub conversion. Importantly, they are applied only to the last pandoc conversion process, and not to the AST conversion processes.

Preprocessors however are supported but advised against, as the default preprocessor converts the input files to pandoc native.

Processor arguments are fully supported, and operate on the pandoc command.

## CustomPreprocessors conversion 

Custom preprocessors conversion uses preprocessors to convert to a common format, then concatenating the output to the final file. There is no template file for custom preprocessors conversion.

To convert via this method, you must define one or more custom preprocessor. There are no default custom preprocessors for this conversion, which means the custom preprocessors must cover each file type converted.

Lua filters are supported for this conversion, and only get added for pandoc processes.

Custom processors are **not** supported, as there is no unified processing step.

## CustomProcessor conversion

Similarly to custom preprocessors conversion, custom processors conversion does not have a template file, but instead converts all input files to pandoc AST before combining them to one large AST mega file and converting that with a custom processor.

To convert via this method, you must set a custom processor with arguments for the target file type. The target file name is automatically appended to the process and must not be included in the arguments.

Lua filters behave identically to custom processor conversion, applying only to the final pandoc process.

Similarly, preprocessors are supported but advised against unless entirely necessary.

Custom processor arguments are mandatory.
