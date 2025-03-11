# LICENSE

```
MIT License

Copyright (c) 2025 Lena Tauchner

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

# The what?

> *If you want to skip the funny written explanations, skip to the*
> *[Usage](#usage) section.*

Well, that's a good question. Ask it later.

Jk, of course you may ask it now.
TiefDown is a project format I made up to make
it easier to convert my markdown files into something pretty.
As a matter of fact, this documentation is managed by a TiefDown project!

The important thing is that this isn't a markdown parser, replacement or
anything like that. It's a project format, and it's not even a format, it's
pretty much just a manifest file and an executable.

## Why?

I wonder myself every day. 
But alas, I should know, I wrote this cluster\*\*\*\* so let
me explain. The initial concept was born from pain (as many are).
I was pretty tired of exporting my markdown files, then converting them, 
overwriting my old files, then converting them again, overwriting all history 
in the process. It was just... a mess.

So I did what any sane person would do: I learned Python.

Well, I'm being facetious. I didn't "learn Python," I just expanded my
capabilities to calling programs from the command line.

So my script, at first, just called Pandoc, then pdflatex, and then
pdflatex again for good measure. It created a PDF, overwriting my old one.
It was basically just converting a single markdown file into a PDF with a
basic TeX template (in my case, LiX Novel).

Then I realized that writing a 40-chapter story in a single markdown file was
even dumber than whatever I made in Python. So I added a little combination
logic. In the process, I had to write Lua filters as well, and then I added
versioning, and then I added conversion to multiple different PDFs, and then I
added EPUB support and—you know what? That was a dumb idea. The Python script
soon reached 200 lines of code, which was untenable.

So yeah, I decided to make a new book. And of course - _**everything**_ broke.
Instantly. I had to copy and paste things, adjust my Python script, rewrote it
a bit, and boom - suddenly I had two different projects with different processes,
different outputs, different versions, different everything.

And then... I started a third book. Aaaand the Python script didn't really fulfill
my needs, so I rewrote it in Bash. But worse.

I thought I had it all figured out. With Python. Then Bash. Then I started a short
story and lost my \*\*\*\*\*\*\* mind.

## How, oh wise programmer, did you solve this problem?

I'm glad you asked! I'm glad. I... I hope you asked?
Well, regardless of whether or not you did, I'll tell you.

I learned Rust.

For real this time, I learned a completely new programming language just for this.
But there was a reason, or a few rather:

1. I wanted cross-platform support.
2. I wanted a single executable.
3. I needed a language with good CLI support because, believe it or not, I'm *awful* at GUIs.
4. I'm crazy.

These reasons led me to two options: Python, a language I was somewhat familiar
with but didn't particularly enjoy writing in, and Rust, a language I had never
written in before but was very interested in.

Evidently, I chose Rust.

So I started: a CLI interface, command-line calls, and so on. Here's the rundown
of how it works internally:

- You initialize a project with `tiefdownconverter init`. This creates a few bits and
  bobs, but importantly the `manifest.toml` file. This contains all the
  information needed to actually manage and convert the project.
- You can then manipulate the project, and so on.
- When you add your markdown files to the Markdown directory, running
  `tiefdownconverter convert` will do a few things:
  - Create a new folder for the current compilation. That way, you have a
    history.
  - Combine all the markdown files into one megafile called `combined.md`.
  - Run Pandoc conversion to TeX, EPUB, or Typst. This uses Lua filters that are
    defined in the `manifest.toml` file.
  - Run XeLaTeX on all TeX templates, Typst on all Typst templates, and so on. It even 
    supports EPUB conversion.
  - Copy the files around so you end up with your output files in the right places.

Isn't that simple?

It isn't. But oh well. We've got a lot of work to do on this, and if you're
interested, don't shy away from the [Contributing](#contributing) section!

## So, what's the point?

Really? Making my life easier. I wanted to export my novel as a PDF in A4, 8x5in, so on.
If I can make your life easier as well, then I'm the happiest woman alive.

## Use Cases

So where does TiefDownConverter actually come in handy? Well, anywhere you need Markdown 
to turn into something nice without manually fiddling with formats every time. Here are 
a few scenarios where it saves the day:

- **Writing Books** - Markdown is great for writing, but formatting a 300-page novel? Not so much. TiefDown handles that for you.
  Well, at least the part where you need to convert stuff, you still need to write out your templates.
- **Technical Documentation** - Software projects need structured documentation, and TiefDown makes sure it's consistent.
  Case and point, this documentation is managed as a TiefDown project.
- **Multi-format Exports** - Need a A4 PDF, a Book PDF, a letter PDF, EPUB, so on? TiefDown can generate them all from the same source.

Basically, if your workflow involves Markdown and you’re sick of manually converting everything, TiefDown is your new best friend.
