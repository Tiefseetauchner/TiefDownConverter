# Welcome to TiefDown

> This document prescribes and describes how to use and work with TiefDown Projects. Crucially, it does not document TiefDownLib or TiefDownConverter. Check out the corresponding documentation from the main page: [https://tiefseetauchner.github.io/TiefDownConverter](https://tiefseetauchner.github.io/TiefDownConverter).

TiefDown is a project format for converting structured markdown inputs into deterministic, structured outputs. A TiefDown project consists of markdown folders, a manifest, and templates, which are processed through a defined conversion pipeline to produce documents such as PDF or EPUB.

Document conversion pipelines tend to grow complex quickly when combining multiple inputs, templates, metadata, and processors. TiefDown exists to make these pipelines explicit, reproducible, and manageable.

TiefDown allows full control over the conversion process, from simple Pandoc-based workflows to complex pipelines involving custom preprocessors, injections, and multiple output formats, while maintaining deterministic results.