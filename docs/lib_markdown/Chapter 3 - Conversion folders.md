# Conversion folders

The conversion folder is where inputs are collected and processed for a run. A
new, time-stamped folder is created inside the project whenever a conversion
starts. If a [markdown project](#markdown-projects) specifies an `output`
directory, TiefDown uses that as the base for that project’s conversion folder.
Old conversion folders can be removed manually with the clean command or
automatically with [smart clean](#smart-clean).

To understand what goes into the folder, it helps to look at the conversion
pipeline (see the overview diagram):

![Workflow](./resources/architecture.jpg)

1) Input discovery and ordering: TiefDown scans the markdown project directory,
   orders files by the first number in the filename (e.g. `Chapter 10 - …`),
   and recurses into similarly numbered subfolders, preserving their order.

2) Preprocessing by extension: Inputs are grouped by file extension. For each
   group, TiefDown selects the matching preprocessor for the active template
   (either a default or a custom one filtered by extension) and runs it in the
   conversion folder. The stdout from each run is captured.

3) Combined output: The captured outputs are concatenated and written to the
   template’s configured `preprocessors.combined_output` file (typically
   `output.tex` for LaTeX or `output.typ` for Typst). Your template includes
   this file.

4) Metadata files: TiefDown generates `metadata.tex` or `metadata.typ` (only if
   they don’t already exist) based on `[shared_metadata]`, any project-specific
   metadata, and your optional [metadata settings](#metadata-settings).

5) Template processing: Depending on the template type, TiefDown runs XeLaTeX
   (twice) or Typst on the template file in the conversion folder, optionally
   passing arguments from a named [processor](#custom-processors). EPUB templates
   invoke Pandoc directly. `CustomPreprocessors` templates copy the combined
   output as-is to the final destination. `CustomProcessor` templates run a
   final Pandoc invocation reading the combined Pandoc Native input and passing
   the configured processor arguments.

6) Finalization: The produced artifact (e.g. a PDF or EPUB) is then copied to
   the markdown project’s configured output path.
