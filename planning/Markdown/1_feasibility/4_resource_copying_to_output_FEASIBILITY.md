## Resource Copying to Output — Feasibility Study

### Abstract

TiefDownConverter copies declared resources from the markdown project directory into the intermediate build directory during conversion, but these resources are not propagated to the final output directory. Users building websites or other asset-dependent targets that load resources dynamically at runtime must either copy images, stylesheets, and navigation files manually after each build, or author a dedicated Preprocessor to handle this. This study evaluates the feasibility of adding a configuration option to copy declared resources — and optionally generated navigation metadata files — directly to the output directory as part of the standard conversion pipeline.

---

### 1. Background and Motivation

The existing `resources` field on a `MarkdownProject` (type `Option<Vec<PathBuf>>`) specifies files and directories that live inside the markdown source directory and are needed during conversion. The `copy_resources` function in [core/src/conversion.rs](core/src/conversion.rs) copies them into the intermediate build directory alongside the compiled template output. After conversion completes, `convert_template` copies only the primary output file (or directory, for `multi_file_output`) to the project's output directory. Resources are left behind in the timestamped build directory and discarded when smart-clean removes old builds.

This creates a friction point for any workflow that produces output consumed outside TiefDownConverter's build directory — most obviously HTML websites, where the browser resolves image and stylesheet URLs relative to the HTML file's location. A user who builds a multi-page documentation site using the `website` preset (feasibility study 3) and embeds a local image in a markdown page will find that image missing from the output directory after every build.

The two current mitigations are:

1. **Manual post-build copy** — fragile, error-prone, and breaks incremental workflows.
2. **Custom Preprocessor** — functional but disproportionately complex for the task of copying files; it requires understanding the preprocessor system and authoring a shell script or binary that serves as a no-op conversion step with a side-effect of copying files.

---

### 2. Scope of the Proposed Feature

Three distinct categories of artefact are candidates for output-directory propagation:

| Category | Description | Currently handled? |
|---|---|---|
| **Declared resources** | Files/directories listed in `resources` on a `MarkdownProject` | Copied to build dir only |
| **Generated nav metadata** | `nav.yml` / `nav.json` produced by the `meta_gen` pipeline | Written to build dir only |
| **Template assets** | Files from the `template/` directory (CSS, HTML fragments) | Copied to build dir only |

This study addresses category 1 and 2. Category 3 is a separate concern: template assets are shared infrastructure and their propagation has different semantics (they may conflict across markdown projects sharing an output directory).

---

### 3. Use Cases

#### 3.1 Static Website with Local Images

A user maintains a documentation site in TiefDownConverter. Pages reference images stored in `Markdown/images/`. The generated HTML files are served by a static host (GitHub Pages, Nginx, a local development server). After each `tdc convert`, the output directory must contain both the HTML files and the `images/` directory at the same relative path. Today this requires a post-build script.

This use case is real, recurring, and has no clean solution within the current feature set. It directly affects users following the `website` preset workflow introduced in feasibility study 3.

#### 3.2 JavaScript or CSS Loaded Dynamically

A custom HTML template references a `prism.js` file and a `custom.css` stored alongside the markdown sources for version-control convenience. The conversion pipeline produces HTML that `<link>`s or `<script src>`s these files by relative path. Resource propagation would ensure they arrive in the output directory without user intervention.

This use case exists but is narrower than 3.1: users who are sophisticated enough to embed custom JS/CSS in their templates are typically also capable of writing a post-build copy step. It is a real convenience gain, not a blocker.

#### 3.3 Navigation Metadata for Client-Side Rendering

A website uses a lightweight JavaScript navigation renderer that reads `nav.json` at runtime to build a sidebar. TiefDownConverter's `meta_gen` pipeline already produces this file in the build directory. Propagating it to the output directory would close the loop for this pattern.

This use case is plausible but currently limited to users who have already built around TiefDownConverter's `meta_gen` feature, which is itself relatively new. It is worth supporting but not the primary motivation.

#### 3.4 Non-Web Outputs

For LaTeX/PDF or EPUB outputs, resources are embedded into the final document by the conversion process. There is no meaningful output-directory propagation to perform — the output is a single self-contained file. The feature would silently apply to these output types too, doing unnecessary file copies. This is harmless but worth noting: copying images to the same directory as a PDF output is useless and could surprise users.

---

### 4. Design Options

#### 4.1 Option A: Boolean flag `copy_resources_to_output` on `MarkdownProject`

Add `copy_resources_to_output: Option<bool>` to the `MarkdownProject` struct. When `true`, after `convert_template` completes, the same resources already copied to the build directory are also copied to the project's output directory.

**Pros:** Minimal manifest surface area; opt-in; no breaking changes; isolated to the existing `copy_resources` call site.

**Cons:** The flag applies to all templates that convert the markdown project. A project that produces both PDF and HTML output from the same markdown project cannot selectively enable resource copying only for the HTML template — enabling the flag causes useless copies alongside the PDF as well.

#### 4.2 Option B: Separate `output_resources` list on `MarkdownProject`

Add `output_resources: Option<Vec<PathBuf>>` as a distinct list. Resources in `resources` are copied to the build directory only; resources in `output_resources` are copied to both.

**Pros:** Precise control; avoids propagating resources that are only needed during conversion (e.g., a `.bib` file for BibTeX). Cleanly extensible to cover nav metadata by including the generated file path.

**Cons:** Two lists to maintain; potentially confusing relationship between `resources` and `output_resources` for new users. Inherits the same template-granularity problem as Option A — still applies to all templates.

#### 4.3 Option C: Boolean flags on `Template`

Add `copy_resources_to_output: Option<bool>` and `copy_generated_meta_content: Option<bool>` to the `Template` struct. Each template independently declares whether the markdown project's resources and the generated nav metadata file should be propagated to the output directory after that template's conversion completes.

**Pros:** Correct granularity. The decision is made at the template level, which is exactly where the output type is known. A project with a `tex` template and an `html` template can enable resource copying only on the latter. The two flags are independently controllable — a template can propagate resources without propagating nav metadata, or vice versa. No changes to `MarkdownProject` are required.

**Cons:** A user who wants resource copying for all templates must set the flag on each template entry. In practice this is uncommon — the use case is almost always tied to a specific output type — so this is a minor inconvenience in an edge case, not a real friction point for the primary use case.

#### 4.4 Recommendation

**Option C** is the correct design. The fundamental question — "should resources be present alongside the output?" — is an output-type concern, not a markdown-project concern. Placing the decision on `Template` makes this explicit and eliminates the meaningless-copy problem for non-web outputs without requiring users to understand the subtlety. The `copy_generated_meta_content` flag follows the same logic: nav metadata is only meaningful when the output can consume it dynamically, and that is a property of the template, not the source.

---

### 5. Implementation Complexity

The core implementation is straightforward:

1. Add `copy_resources_to_output: Option<bool>` and `copy_generated_meta_content: Option<bool>` to `Template` in [core/src/manifest_model.rs](core/src/manifest_model.rs).
2. In the `convert` loop in [core/src/conversion.rs](core/src/conversion.rs), after `convert_template` succeeds, check the flags on the resolved template and call new helper functions that mirror the existing `copy_resources` logic but target `project.join(markdown_project.output)` instead of the build directory.
3. No manifest version bump is required because the new fields are optional and additive; older versions will silently ignore them on round-trip.

The destination path requires care: the output directory must be created before copying, which `convert_template` already ensures. The copy itself is a near-verbatim reuse of the existing `copy_resources` implementation.

Nav metadata file propagation is slightly more involved: the file is generated inside the build directory by the `meta_gen` pipeline and its path is not available until after `convert_template` returns. The path is deterministic (`nav_output` from `MetaGenerationSettings`, defaulting to a fixed filename), so it can be reconstructed and copied without modifying the converter return type.

| Concern | Effort |
|---|---|
| Manifest model change (`Template`) | Trivial |
| `convert` loop addition | Low |
| Path construction and safety checks | Low |
| Nav metadata propagation | Low–Medium |
| Manifest upgrade (not required) | None |
| Testing | Low–Medium |

---

### 6. Testability

#### 6.1 Unit Tests

The feature has no meaningful unit-testable units beyond what already exists — the logic is a conditional file copy with a path. Unit tests for path construction are of marginal value.

#### 6.2 Integration Tests

Integration tests are the appropriate vehicle:

| Test | Scenario |
|---|---|
| `copy_resources_to_output_copies_image` | Template has flag true; resource declared on markdown project; image appears in output dir |
| `copy_resources_to_output_false_does_not_copy` | Flag false (default); resource absent from output dir |
| `copy_resources_to_output_directory_resource` | Resource is a directory; entire subtree appears in output dir |
| `copy_generated_meta_content_propagates_nav_file` | `meta_gen` configured; `copy_generated_meta_content` true on template; nav file appears in output dir |
| `copy_resources_selective_by_template` | Project has two templates (HTML and LaTeX); flag true on HTML template only; resource present in output dir, no spurious copy alongside PDF |

The last test directly validates the core advantage of Option C over Options A and B.

Existing conversion integration tests are unaffected — both flags default to `false` and the new code paths are not entered.

The test infrastructure already supports temporary project directories with controllable manifests; new tests follow the established pattern in [cli/tests/convert_integration_test.rs](cli/tests/convert_integration_test.rs).

---

### 7. Risks and Limitations

#### 7.1 Meaninglessness for Non-Web Outputs

Copying images next to a `.pdf` is useless. Because the flags live on `Template`, a user must simply not set them on non-web templates — a natural constraint, since a LaTeX template author has no reason to seek out these flags. The risk of accidental misuse is lower than it would be with a `MarkdownProject`-level flag, where the template type is not in view.

#### 7.2 Output Directory Pollution

If a user's output directory is the project root (i.e., `output = "."`), resource propagation writes files directly into the project root. This may interfere with version control or project structure expectations. This is not new — the existing output-file copy already has this behaviour — but adding resource directories amplifies the surface area.

#### 7.3 Overwriting User-Modified Files

If a user manually edits a resource in the output directory (e.g., a CSS file they have locally patched), the next `tdc convert` will overwrite it with the source version. This is consistent with how TiefDownConverter treats all output but may surprise users who treat the output directory as a working directory.

#### 7.4 Nav Metadata Path Coupling

Propagating the nav file requires knowing its output path, which is determined by `MetaGenerationSettings.nav_output`. If this field is absent, TiefDownConverter must apply the same defaulting logic used during generation. A mismatch between the generation default and the propagation default would silently result in the wrong file being copied or no file being found. This is a low-probability but non-trivial bug surface that requires careful implementation.

---

### 8. Conclusion

Resource copying to the output directory is a genuine usability gap with a direct, recurring impact on the primary non-document use case: HTML website generation. The implementation effort is low, the design is additive and non-breaking, and the feature composes cleanly with the existing `resources` and `meta_gen` systems.

The use case is real but narrow. The majority of TiefDownConverter users produce PDF or EPUB output where this feature is irrelevant. For the website use case — which feasibility study 3 actively encourages — the friction is significant enough to warrant addressing.

**Recommendation: Implement using Option C (flags on `Template`).** Two boolean fields — `copy_resources_to_output` and `copy_generated_meta_content` — are added to the `Template` struct. This places the decision at the correct granularity, eliminates meaningless copies for non-web outputs without requiring user awareness of the subtlety, and requires no changes to the `MarkdownProject` model. Both flags ship in the same iteration; neither depends on the other.
