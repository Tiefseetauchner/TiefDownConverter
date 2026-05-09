# TiefDownConverter

[![GitHub Release](https://img.shields.io/github/v/release/tiefseetauchner/tiefdownconverter?sort=semver&style=for-the-badge)](https://github.com/Tiefseetauchner/TiefDownConverter/releases)
[![Crates.io Version](https://img.shields.io/crates/v/tiefdownconverter?style=for-the-badge)](https://crates.io/crates/tiefdownconverter)
[![](https://dcbadge.limes.pink/api/server/https://discord.gg/EG3zU9cTFx)](https://discord.gg/EG3zU9cTFx)

![Fibschiiiiiiiiiiiii_smol](https://github.com/user-attachments/assets/f964e2b3-c728-4547-bd29-1ca6c861bf01)

**Write once. Convert forever. Stop touching that Pandoc command.**

---

## The pitch

You have Markdown files. You want a PDF. Maybe two PDFs — one A4, one for print. Maybe also an EPUB. Maybe a Typst version because you're that kind of person.

Without TiefDownConverter, this looks like:

```sh
pandoc Chapter\ 1.md Chapter\ 2.md Chapter\ 3.md \
  --lua-filter=chapter_filter.lua \
  -o output.tex && \
xelatex -interaction=nonstopmode template.tex && \
xelatex -interaction=nonstopmode template.tex  # yes, twice, don't ask
```

...times however many formats you need, manually, every time, forever.

With TiefDownConverter, you set up a project once and then:

```sh
tiefdownconverter convert
```

That's it. All your formats, all your templates, one command. Go make a coffee.

---

## Is this for you?

Probably, if:
- You write long-form documents in Markdown (novels, documentation, academic papers, anything with chapters)
- You need the same source to produce multiple output formats
- You've ever copy-pasted a Pandoc invocation and immediately lost track of it

Probably not, if you need a GUI, a WYSIWYG editor, or something you can hand to someone who's never opened a terminal. This tool has opinions and it will share them with you.

---

## Feature highlights

**Multi-format output from one source**  
Define as many output templates as you want — LaTeX PDFs, Typst PDFs, EPUBs — and convert them all at once, or selectively with `--templates` or `--profile`. Your source files don't change; only the outputs do.

**Chapter-aware file ordering**  
Drop your files into the input directory named `Chapter 1.md`, `Chapter 2.md`, and so on. TiefDownConverter figures out the order. Subdirectories work too, recursively, following the same numbering logic. No config required — it just does the obvious thing.

**Multiple markdown projects in one TiefDown project**  
Managing a series? Each book can be its own markdown project under a single TiefDown project, with shared templates and metadata (publisher, series name) and per-book metadata (title, author). One `convert` command handles all of them.

**Lua filter support**  
Pandoc's Lua filters slot right in. Attach them per-template so your A4 PDF and your EPUB can behave differently without duplicating anything. This is where the real power is, if you know what you're doing with Pandoc.

**Conversion profiles**  
Tired of typing `--templates lix_novel_a4.tex,lix_novel_book.tex` every time? Save that as a profile and use `--profile print` instead. Profiles can also be set as the default for a markdown project.

**Preprocessors and custom processors**  
If the built-in Pandoc invocation isn't enough, define your own. Add `--listings`, change output formats, use `cat` for passthrough — whatever Pandoc accepts, you can wire in. For formats that can't be concatenated (like docx), the custom processor path handles multi-file merging via Pandoc's native AST.

**Injections**  
Need a different chapter 3 depending on whether you're outputting print or web? Header/body/footer injections let you splice per-template content into the preprocessing step without maintaining separate source trees.

**Shared metadata**  
Set author, publisher, series info once at the project level. Override it per markdown project where needed. The metadata gets written to `metadata.tex` or `metadata.typ` and is accessible in your templates via `\meta{}` (LaTeX) or importing `meta` (Typst).

**Smart cleaning**  
Conversion directories stack up. Smart clean keeps only the N most recent ones (configurable, default 5). Enable it at init time or toggle it later — it runs automatically if you want, or manually with `tiefdownconverter project smart-clean`.

---

## Getting started

### Install

```sh
cargo install tiefdownconverter
```

Or grab a prebuilt binary from [GitHub Releases](https://github.com/Tiefseetauchner/TiefDownConverter/releases).

### Dependencies

You need these installed and on your PATH:

- [Pandoc](https://pandoc.org/)
- A TeX distribution with XeLaTeX (TeX Live, MiKTeX, MacTeX)
- [Typst](https://typst.app/) — optional, only needed for Typst templates

**Linux (Debian/Ubuntu):** `sudo apt install pandoc texlive-xetex` + Typst from [typst.app](https://typst.app/)  
**Windows:** `winget install miktex pandoc typst`  
**Mac:** MacTeX + Pandoc from [pandoc.org](https://pandoc.org/), Typst if needed

Run `tiefdownconverter check-dependencies` to verify everything's in place.

### Your first project

```sh
mkdir my-novel
cd my-novel
tiefdownconverter init -t lix_novel_a4.tex
```

This creates a project with a LiX-backed A4 template, a `Markdown/` input directory, and a `manifest.toml`. Drop your chapter files into `Markdown/`, named with a number somewhere in the filename (`Chapter 1.md`, `01 - Introduction.md`, whatever works for you), then:

```sh
tiefdownconverter convert
```

A PDF appears. That's the whole loop.

### Adding more output formats

```sh
tiefdownconverter project templates lix_novel_book.tex add
tiefdownconverter convert
```

Now you have two PDFs. Adding EPUB or Typst works the same way.

---

## Available preset templates

| Template | Description |
|---|---|
| `template.tex` | Basic LaTeX template, good starting point |
| `booklet.tex` | Booklet layout |
| `lix_novel_a4.tex` | LiX Novel, A4 paper |
| `lix_novel_book.tex` | LiX Novel, 8×5in print |
| `template_typ.typ` | Basic Typst template |
| `default_epub` | EPUB output |

LiX templates will prompt you to auto-download the required [LiX](https://github.com/NicklasVraa/LiX) `.sty` and `.cls` files on first use. Say yes.

---

## Project structure

```
my-novel/
├── manifest.toml          # the whole project lives here
├── Markdown/
│   ├── Chapter 1.md
│   ├── Chapter 2.md
│   └── ...
└── template/
    ├── lix_novel_a4.tex
    └── meta.tex
```

The `manifest.toml` tracks everything — templates, filters, preprocessors, metadata, profiles. You can edit it directly or use the `tiefdownconverter project` subcommands if you'd rather not think about TOML.

---

## Documentation

Full documentation is in [docs/docs.pdf](https://github.com/Tiefseetauchner/TiefDownConverter/blob/main/docs/docs.pdf) (generated with TiefDownConverter itself, naturally).

---

## Community & support

- [Discord](https://discord.gg/EG3zU9cTFx)
- [GitHub Issues](https://github.com/Tiefseetauchner/TiefDownConverter/issues)

Contributions welcome. See the Contributing section in the docs for the architecture overview and PR expectations.

---

## License

MIT. See `LICENSE`.

---

## Coffee

[![Buy me a coffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/tiefseetauchner)

---

*Mascot (Fibschiiiiiiiiiiiii) by Finn ♥*