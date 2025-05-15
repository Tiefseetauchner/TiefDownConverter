# Introduction

> For the documenation of the library, see [docs.rs](docs.rs/tiefdownlib).
>
> For the documentation of the CLI, see [TiefDownConverter](https://tiefseetauchner.github.io/TiefDownConverter/cli).

This is the documentation for the TiefDown concepts. This won't explain the
library or the CLI usage, but rather function as an introduction to the
basics of TiefDown for users and contributors alike.

## What is TiefDown?

TiefDown is a project format for managing markdown files and converting them
to other formats. It's not a markdown parser, but rather a project format
and management system.

Importantly, the project is split in a few parts:

- The `manifest.toml` file, which contains all the information needed to
  manage and convert the project.
- The `template` directory, which contains all the templates for the project.
- One or more markdown directories, corresponding to markdown projects.
