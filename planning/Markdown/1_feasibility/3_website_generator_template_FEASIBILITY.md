## Website Generator Template — Feasibility Study

### Abstract

TiefDownConverter is capable of acting as a static site generator (SSG): the TiefDownConverter documentation project itself is built with it, producing multi-page HTML output via `CustomPreprocessors` templates, Pandoc's HTML5 backend, injection-based headers and footers, and a Lua-driven navigation layer. That configuration is, however, the product of accumulated project-specific knowledge — it is not reusable, not documented as a starting point, and involves several non-obvious decisions (injection ordering, Lua navigation generation, multi-file output flags, metadata field conventions). This study evaluates whether shipping a first-class `website` preset — a self-contained, opinionated starting point for documentation sites — is warranted, what it would require, and where the genuine risks lie.

---

### 1. Background and Motivation

The TiefDownConverter README acknowledges that this tool is not aimed at users who want a GUI or a zero-configuration experience. Its power users are writers and developers comfortable with TOML configuration, Pandoc, and Lua. Within that audience, there is a real but narrow sub-population who want to publish documentation sites from Markdown sources: software projects, technical writing workflows, personal wikis.

Today, a user who wants to use TiefDownConverter as an SSG must:

1. Understand `CustomPreprocessors` vs. `CustomProcessor` template types.
2. Understand `multi_file_output`, `output_extension`, and `combined_output` semantics.
3. Understand the injection system (header, body, footer; ordering; that injections are inserted at the Pandoc input stage, not post-processed).
4. Write or adapt HTML injection files manually.
5. Author or copy a navigation Lua filter, and understand how `meta_gen` interacts with it.
6. Understand the metadata field pass-through mechanism (`--metadata title={{title}}`).

None of these steps are individually intractable, but the combination is high enough friction that a motivated user can spend a full day before producing anything that renders correctly. The existing docs project provides a working reference, but it is not structured as a reusable scaffold — it contains TiefDownConverter-specific navigation, content, and metadata.

A `website` preset would lower this barrier by providing a turnkey starting point: sensible defaults, working HTML/CSS, and a manifest skeleton a user can understand and extend without first understanding the full internals.

---

### 2. Proposed Scope

The preset is scoped to what can be shipped as static files inside the TiefDownConverter binary (alongside the existing LaTeX and Typst templates in `core/src/resources/templates/`) or as an `init`-time scaffold written to disk.

The minimal viable preset consists of:

| Artifact | Description |
|---|---|
| `manifest.toml` skeleton | Pre-configured `CustomPreprocessors` template with correct multi-file and single-file output, a navigation filter, and a default profile |
| `template/style.css` | A CSS file providing readable, minimal styling |
| `template/head.html` | Injection: `<head>` open, `<style>` link or inline, `<nav>` open |
| `template/footer.html` | Injection: `</body></html>` close with a simple footer |
| `template/nav.lua` | Lua filter that inserts inter-page navigation links |
| `Markdown/index.md` | Seed content file so conversion runs immediately after init |

A secondary goal, but not a requirement for the first iteration, is a `--site` flag on `tdc init` that invokes the preset and writes these files, as opposed to requiring the user to copy them from documentation.

---

### 3. Current Architecture Fit

#### 3.1 What Already Works

The docs project demonstrates that the full SSG pipeline is functional today:

- `CustomPreprocessors` with `output_extension = "html"` and `multi_file_output = true` produces one HTML file per input Markdown file.
- `CustomPreprocessors` with `combined_output = "index.html"` collapses all inputs into a single page.
- Injections (`header_injections`, `footer_injections`) splice raw HTML around each converted chunk.
- `[templates.meta_gen] feature = "Full"` and `feature = "MetadataOnly"` drive the navigation Lua filter via generated metadata.
- `--metadata title={{title}}` passes project-level metadata into Pandoc per file.

None of these paths require modification to ship a preset.

#### 3.2 What the Preset Must Work Around

Two genuine friction points exist that the preset cannot fully hide:

**Absolute CSS paths.** The docs manifest uses `--css /TiefDownConverter/template/html_template/style.css` — an absolute path hardcoded to the docs project's deployment environment. A general preset cannot hardcode a path that works for every user. Options:

- Inline the CSS directly in `head.html` (eliminates the path problem entirely; increases injection file size; makes CSS harder to override).
- Use a relative path and document that users must serve from the project root or adjust the path.
- Accept this limitation and document it clearly.

Inlining is the least surprising choice for a preset aimed at first-time users.

**Navigation generation.** The existing `navlib.lua` and `nav.md` approach is coupled to TiefDownConverter's own site structure. A general-purpose nav filter needs to operate on whatever files the user provides, constructing links from metadata rather than a hardcoded list. This requires either a generic nav Lua filter (moderate Lua work, ~100 lines) or accepting that navigation in the preset is manual (a `nav.md` the user edits). The latter is simpler and more honest about what TiefDownConverter actually automates.

---

### 4. Usefulness Assessment

#### 4.1 Who Would Use This

The realistic audience is narrow. It is not people who are already using Jekyll, Hugo, or MkDocs — those tools are better SSGs, with themes, plugin ecosystems, live reload, and deployment integrations. The realistic audience is:

- Existing TiefDownConverter users who already produce PDF output and want HTML as an additional format without switching tools.
- Software developers who write documentation in Markdown, already use TiefDownConverter for some output, and want a quick static site without learning a second tool.

This is a useful but small population. TiefDownConverter is not competing with dedicated SSGs on features, and the preset should not pretend otherwise.

#### 4.2 Practical Scenarios

**Software project documentation.** A developer uses TiefDownConverter to produce a PDF reference manual. Adding an HTML profile with the website preset gives them a deployable GitHub Pages site with no new tool dependency. The PDF and HTML share the same Markdown source.

**Personal notes or wiki.** A writer who already uses TiefDownConverter for long-form writing wants to publish a subset of their notes as a simple website. The preset gives them a working starting point in under five minutes.

**TiefDownConverter onboarding showcase.** The preset doubles as a demonstration of the injection and multi-file output systems. Users who initialise a site project and then read the generated `manifest.toml` learn about these features from a working example.

#### 4.3 Limitations That Reduce Usefulness

- No live reload or development server. The user must re-run `tdc convert` and refresh the browser manually. This is a significant ergonomic gap compared to any dedicated SSG.
- No asset pipeline. Images, JavaScript, or other non-Markdown assets must be managed manually and copied into the output directory.
- Navigation is manual unless a generic nav filter is implemented. Users must maintain a `nav.md` or equivalent.
- Deployment is entirely manual. No integration with GitHub Pages, Netlify, or similar services.

These are not bugs — they reflect TiefDownConverter's scope — but they mean the preset solves the initial-configuration problem, not the ongoing-workflow problem. A user who outgrows TiefDownConverter as an SSG will switch to a dedicated tool. The preset should be honest about this in its generated `manifest.toml` comments or in documentation.

---

### 5. Implementation Complexity

| Concern | Effort |
|---|---|
| CSS and HTML injection files | Low — static files, no new code |
| Manifest skeleton | Low — TOML written once, embedded as a resource |
| `tdc init --site` flag (or `--template website`) | Low-Medium — wires into existing init path |
| Generic nav Lua filter | Medium — ~100 lines of Lua, needs testing across file counts |
| Relative CSS path workaround (inlining) | Low — authoring choice, not a code change |
| Documentation | Low-Medium — a page in the existing docs project |

The total implementation effort for the minimal preset (without a generic nav filter) is low. The nav filter adds moderate effort and testing surface. The `tdc init` integration is the most likely place for edge-case bugs (path handling, existing file conflicts).

---

### 6. Testability

#### 6.1 What Can Be Tested Automatically

- `tdc init --site` (or equivalent) creates the expected files in a temp directory.
- The generated `manifest.toml` parses correctly and passes manifest validation.
- `tdc convert` on the generated project succeeds and produces at least one `.html` file.
- The generated HTML contains the expected `<nav>` element (structural check, not visual).

#### 6.2 What Cannot Be Tested Automatically

Visual styling correctness requires a browser. The CSS will not be pixel-perfect by default across browsers. A test that checks for the presence of structural HTML elements is the practical limit of automated verification. Styling regressions can only be caught by manual inspection or a snapshot-based browser test (which is out of scope for this project's test infrastructure).

---

### 7. Risks

**Scope creep.** An SSG preset naturally invites feature requests: themes, syntax highlighting, search, RSS, sitemap generation. The preset must be explicitly documented as an opinionated minimal starting point, not as a competing SSG. If the feature request volume is high, the cost of maintaining parity with user expectations will exceed the benefit.

**CSS maintenance burden.** Shipping a CSS file means owning it. Any future design change requires updating the file and documenting the change. If the preset's styling becomes dated or is perceived as low quality, it reflects on TiefDownConverter's presentation. The existing docs CSS (`head.html`) is polished but non-trivial in size. Shipping it as-is is reasonable but means any changes to the docs styling diverge from the preset unless they are kept in sync.

**False advertising.** The preset may set expectations that TiefDownConverter cannot meet as a general SSG. Users who initialise a site project and then discover that navigation requires manual maintenance, or that there is no dev server, may be disappointed. The docs and `manifest.toml` comments must set expectations correctly.

---

### 8. Comparison to Alternatives

A user who wants a static site from Markdown has better-supported alternatives: MkDocs (Python, excellent documentation tooling), Hugo (Go, fast, large theme ecosystem), or Jekyll (Ruby, GitHub Pages native). These tools are specifically designed for this use case and will outperform a TiefDownConverter preset on nearly every SSG-specific dimension.

The preset's value proposition is narrow: it is useful precisely for users who are already in the TiefDownConverter ecosystem and want HTML as one additional output format, not as the primary product. It is not a reason to choose TiefDownConverter as an SSG in a greenfield project.

---

### 9. Conclusion

A website generator template preset is technically trivial to implement in its minimal form — the pipeline already works, the assets exist in the docs project, and the manifest skeleton is a one-time authoring task. The genuine questions are whether the narrowness of the target audience justifies the ongoing CSS and documentation maintenance cost, and whether shipping an SSG preset is honest about TiefDownConverter's capabilities.

**Recommendation: Implement the minimal preset, with caveats.** The implementation cost is low enough that it earns its place as an onboarding aid and a multi-format complement for existing users. It should be explicitly framed as a starting point for users who already use TiefDownConverter, not as a general SSG recommendation. The generic nav filter should be deferred to a second iteration — manual navigation via a `nav.md` file is good enough for the initial preset and avoids the ongoing Lua maintenance surface. Do not add a live-reload dev server, asset pipeline, or deployment integration: if those become requested, the correct answer is to point users at a dedicated SSG.
