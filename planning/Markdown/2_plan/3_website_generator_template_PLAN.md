## Website Generator Template — Implementation Plan

### Overview

Add a `website` preset to `tdc init` that scaffolds a working static site project: a `manifest.toml` configured for multi-page HTML output, inlined CSS styling, HTML injections for a header and footer, a manual navigation file, and a seed content page. No new conversion pipeline code is required. The preset ships as static resource files embedded in the binary alongside the existing LaTeX and Typst templates.

---

### 1. Deliverables

| File | Destination (relative to new project root) |
|---|---|
| Preset manifest skeleton | generated into `manifest.toml` at init time |
| `head.html` | `template/head.html` |
| `footer.html` | `template/footer.html` |
| `nav.md` | `template/nav.md` |
| `index.md` | `Markdown/index.md` |

CSS is inlined into `head.html` to avoid absolute-path issues. No separate `.css` file is shipped.

---

### 2. Resource Files

#### 2.1 `head.html`

A complete `<!DOCTYPE html>` … `<body><nav>` fragment with inlined CSS. Base the styling on the existing `docs/template/head.html` (the glassmorphism card layout), stripping TiefDownConverter-specific navigation links. The nav element closes immediately after its opening tag; the nav Lua filter or the `nav.md` injection fills it.

The file ends before `</nav>` so the nav injection content lands inside the `<nav>` element:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{{title}}</title>
  <style>
    /* ... inlined CSS ... */
  </style>
</head>
<body>
  <nav>
```

#### 2.2 `nav.md`

A Markdown file containing navigation links. The user edits this file to add links to their pages. It is included as a body injection (injected between `head.html` and the converted content, so it appears inside the `<nav>` element):

```markdown
[Home](index.html) | [Page 2](page-2.html)
```

Users add new entries as they add new Markdown files. This is intentionally manual — auto-generated navigation is deferred.

#### 2.3 `footer.html`

Closes the body and adds a minimal footer:

```html
  </article>
</main>
<footer>
  <p>Generated with <a href="https://github.com/Tiefseetauchner/TiefDownConverter">TiefDownConverter</a></p>
</footer>
</body>
</html>
```

#### 2.4 `Markdown/index.md`

A seed file so `tdc convert` succeeds immediately after init:

```markdown
# My Site

Welcome to my site. Edit this file to get started.
```

---

### 3. Manifest Skeleton

The generated `manifest.toml` for a website project:

```toml
version = 6
smart_clean = true
smart_clean_threshold = 5

[[custom_processors.preprocessors]]
name        = "HTML"
cli_args    = [
  "--to", "html5",
  "--metadata", "title={{title}}",
  "--metadata", "author={{author}}",
]

[[injections]]
name  = "site-header"
files = ["head.html", "nav.md"]

[[injections]]
name  = "site-footer"
files = ["footer.html"]

[[markdown_projects]]
name   = "Site"
path   = "Markdown"
output = "site"

[shared_metadata]
title  = "My Site"
author = ""

[[templates]]
name              = "HTML Multi-Page"
template_type     = "CustomPreprocessors"
multi_file_output = true
output            = "site"
header_injections = ["site-header"]
footer_injections = ["site-footer"]

[templates.preprocessors]
output_extension = "html"
preprocessors    = ["HTML"]

[[profiles]]
name      = "default"
templates = ["HTML Multi-Page"]
```

The `nav.md` is in `header_injections` so it lands inside the `<nav>` element opened by `head.html`.

---

### 4. `tdc init` Integration

#### 4.1 New Flag

Add `--site` to the `init` subcommand. When passed, `init` writes the website preset instead of the default no-template project.

```
tdc init --site [--output <dir>]
```

`--site` and `-t` / `--templates` are mutually exclusive; the CLI should error if both are provided.

#### 4.2 Implementation Path

The existing `init` path in `cli/src/main.rs` (or wherever init is dispatched) calls into `core` to write a `manifest.toml` and copy template files. The `--site` branch:

1. Creates the project directory (same as today).
2. Writes `manifest.toml` from the website skeleton (either a hard-coded struct serialized to TOML, or a raw string constant).
3. Copies `head.html`, `footer.html`, `nav.md` from embedded resources into `template/`.
4. Writes `Markdown/index.md`.

The resource files are embedded via `include_str!` macros in `core/src/resources/` alongside the existing template files.

#### 4.3 Existing Init Code Impact

The `--site` flag is additive. The existing `init` behavior is unchanged. No existing tests are affected.

---

### 5. Resource Embedding

Add to `core/src/resources/templates/` (or a new `core/src/resources/website/` directory):

```
core/src/resources/website/
├── head.html
├── footer.html
├── nav.md
└── index.md
```

Embed with `include_str!` constants in a new `core/src/website_preset.rs` module. This module exposes a single function:

```rust
pub fn write_website_preset(project_path: &Path) -> Result<()>
```

That function writes the four files to their destinations and returns the manifest struct for the caller to serialize.

---

### 6. Test Plan

#### 6.1 Unit Tests

| Test | Assertion |
|---|---|
| `website_preset_writes_all_files` | After calling `write_website_preset`, all four expected files exist at the correct paths |
| `website_manifest_parses` | The website manifest constant deserializes without error using the existing manifest parser |
| `init_site_and_template_flags_are_exclusive` | Passing `--site -t template.tex` to the CLI exits non-zero with an appropriate error message |

#### 6.2 Integration Tests

| Test | Assertion |
|---|---|
| `init_site_produces_convertible_project` | `tdc init --site` in a temp dir; `tdc convert` succeeds; `site/index.html` exists |
| `generated_html_contains_nav_element` | `site/index.html` contains a `<nav>` tag |
| `generated_html_contains_footer` | `site/index.html` contains a `<footer>` tag |

The integration tests require Pandoc to be available on the test runner's PATH. These tests should be gated behind the same feature flag or environment variable used by other integration tests that invoke external tools.

---

### 7. Documentation

Add a "Getting started: documentation site" page to the TiefDownConverter docs project covering:

- When to use the website preset vs. a dedicated SSG (be direct: dedicated SSGs are better for complex sites).
- How to add pages (create a new `.md` file, add a link to `nav.md`).
- How to customize styling (edit the inlined CSS in `head.html`).
- How to deploy to GitHub Pages (brief — just point to GH Pages docs; TDC does not automate this).

---

### 8. Explicitly Out of Scope

The following are not part of this implementation and should be declined if raised as follow-up issues without a separate feasibility study:

- Live-reload development server.
- Automatic navigation generation from file structure.
- Asset pipeline (image copying, minification).
- Deployment integration (GitHub Actions, Netlify).
- Multiple themes or CSS customisation flags.

---

### 9. Delivery Sequence

1. Author `head.html`, `footer.html`, `nav.md`, `index.md` resource files. Convert locally with `tdc` to verify the HTML renders correctly in a browser.
2. Add `core/src/resources/website/` with embedded constants; add `core/src/website_preset.rs` with `write_website_preset`; unit tests (red → green).
3. Add `--site` flag to `init` subcommand in `cli`; wire to `write_website_preset`; integration tests (red → green).
4. Add `--site` / `-t` mutual-exclusion guard; test.
5. Add documentation page to the docs project.
