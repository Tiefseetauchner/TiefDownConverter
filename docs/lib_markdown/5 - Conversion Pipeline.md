# Conversion Pipeline

The conversion pipeline, at its heart, is a simple queue of template -- markdown project pairs that get processed one by one. Inside the conversion process, there's the following steps:

- Creation of scratch directory
- Copying of templates and resources
- Merging of metadata
- Copying markdown files
- Running conversion
- Copying output files

## Queueing System

Before a document can be converted, a queue must be created. The template -- markdown project pair computation is done in TiefDownLib as a separate step to the conversion. It considers a given list of templates, profiles, and markdown projects, and based on that or, if not provided, the defaults, computes a queue to hand to the conversion pipeline.

There's a few defaults to keep in mind:

- Conversion is run against one manifest/project at a time.
- Markdown projects can be provided as a list.
  - If no markdown project is provided, all markdown projects are converted.
- Either templates or profile can be provided, never both.
- Templates can be provided as a list.
  - If one or more templates are provided, all are converted for all selected markdown projects, regardless of default profile.
  - If no templates are provided, the profile logic sets in.
- One profile can be provided.
  - If a profile is provided, all contained templates are converted for all selected markdown projects, regardless of default profile.
  - If no profile is provided, the default profile logic sets in.
- If neither templates nor profile is provided, the default profile for each markdown project is converted. 
- If there is no default profile for a markdown project, all available templates are converted for this markdown project.

## Scratch Directory

All conversion happens in a temporary scratch directory. This directory is created when conversion is first started, and is named after the current timestamp.

The scratch directory can be automatically removed using [smart cleaning](#smart-clean).

## Template Directory

Next, the template directory is copied to the conversion directory. This is done for each markdown project seperately.

The items of the template directory are put in the markdown project specific folder, in which the primary conversion processes will run.

## Resources

Each markdown project can include resources that may be used by templates. These are, similarly to the template directory, copied to the markdown project specific directory.

## Shared Metadata and Project Metadata Merging Rules

Metadata gets merged after the copying of template and resources. Project specific metadata overrides shared metadata if conflicting.

## Copying Markdown Files

The markdown files are then copied for each conversion task. They are copied to a directory in the markdown project specific directory named after the template.

## Running Conversion

At the heart of the conversion pipeline is, unsurprisingly, the conversion.

First, a converter is decided from the available conversion engines:

- CustomPreprocessor
- CustomProcessor
- Tex
- Typst
- Epub

Each conversion engine has a converter associated with it.

After being decided, the converter is called. Shared in all converters is the retrieval of applicable preprocessors, injections, and input files. Input files are sorted according to the rules [below](#input-file-sorting).

Then, navigation metadata is generated, and it as well as the previously computed metadata is written to the markdown project specific directory in accordance with the [metadata generation settings](#metadata-generation-settings), after which the primary conversion in accordance with the template type is started.

### Input File Sorting

Input file sorting relies on numbers in the to-be-converted input file names. These are parsed and sorted accordingly, with directories with the same number being recursively added after a file of said number. Take the following folder structure:

```
Markdown/
├── Take 1 - Introduction.md
├── Chapter 2 - Usage.md
├── 2 - Usage/
│   ├── Subchapter 1 - Usage detail 1.md
│   └── Subchapter 2 - Usage detail 2.md
└── Asdf 3 - Customisation.md
```

The resulting order would be:

1) Take 1 - Introduction.md
2) Chapter 2 - Usage.md
3) 2 - Usage/Subchapter 1 - Usage detail 1.md
4) 2 - Usage/Subchapter 2 - Usage detail 2.md
5) Asdf 3 - Customisation.md

Alphabetical order is ignored.

### CustomPreprocessor Conversion Engine

The CustomPreprocessor conversion engine follows the following steps:

After the aforementioned steps, the selected preprocessors are run against the appropriate files. Each preprocessor can have a file extension filter, and the most specific filter is chosen for each file, falling back on a preprocessor without filter.

The results are either combined to a singular large file or written to individual files.

### CustomProcessor Conversion Engine

The CustomProcessor conversion engine follows the following steps:

After the aforementioned steps, the input files are run against preprocessors that convert to pandoc AST.

The pandoc native is then combined and written to a file.

Then, it is converted via pandoc in accordance with the parameters from the custom processor.

### TeX Conversion Engine

The TeX conversion engine follows the following steps:

After the aforementioned steps, the input files are converted to LaTeX using the preprocessors and combined.

The combined LaTeX is then written to the combined output file.

Then, XeLaTeX is run against the template file, converting it to PDF. This step is run twice, as XeLaTeX needs to first generate a bibliography.

### Typst Conversion Engine

The Typst conversion engine follows the following steps:

After the aforementioned steps, the input files are converted to Typst using the preprocessors and combined.

The combined Typst is then written to the combined output file.

Then, Typst is run against the template file, converting it to PDF.

### Epub Conversion Engine

The Epub conversion engine follows the following steps:

After the aforementioned steps, the input files are run against preprocessors that convert to pandoc AST.

The pandoc native is then combined and written to a file.

Then, pandoc is run against the AST using the processor arguments.
