# Usage

The basic usage of `tiefdownconverter` is relatively simple.
The difficult part is understanding the templating system and
how to customise it for your usecases. Presets can only do so much.

## Installation

Currently the only way to install `tiefdownconverter` is to either build
it yourself or download a precompiled binary from the
[releases page](https://github.com/tiefseetauchner/tiefdownconverter/releases).
Then just add it to the path and you're good to go. You can of course
also just call it relatively by placing the binary in your project folder or
something like that.

There are a few dependencies that you need to install.

- [Pandoc](https://pandoc.org/): Conversion from Markdown to TeX, Typst and
  Epub.
- A TeX distribution: For converting TeX files to PDF. It has to include
  xelatex.
  - If using [TeX Live](https://www.tug.org/texlive/) you may need to
    additionally install `texlive-xetex` depending on your system.
  - If using [MikTeX](https://miktex.org/), no need to do anything.
- [Typst](https://typst.app/): For converting Typst files to PDF.

Now you should be able to run `tiefdownconverter` from the command line.
You can test it by initialising a test project using `tiefdown init testproject`
and running `tiefdown convert` in the project directory or
`tiefdown convert -p testproject`.

## 
