## Resource Copying to Output — Implementation Plan

### Overview

Add opt-in resource propagation to the conversion pipeline via two boolean flags on `Template`: `copy_resources_to_output`, which copies the markdown project's declared resources to the output directory after conversion completes; and `copy_generated_meta_content`, which does the same for the nav metadata file produced by `meta_gen`. Both flags default to `false`, preserving all existing behaviour. Placing the flags on `Template` rather than `MarkdownProject` ensures that resource copying is controlled at the output-type level — a single project can produce PDF and HTML from the same markdown sources with resource copying enabled only for the HTML template.

---

### 1. Deliverables

| Change | Location |
|---|---|
| `copy_resources_to_output` field on `Template` | `core/src/manifest_model.rs` |
| `copy_generated_meta_content` field on `Template` | `core/src/manifest_model.rs` |
| `copy_resources_to_output_dir` function | `core/src/conversion.rs` |
| `copy_generated_meta_content_to_output_dir` function | `core/src/conversion.rs` |
| Call sites in the `convert` loop | `core/src/conversion.rs` |
| Integration tests | `cli/tests/convert_integration_test.rs` (or a new file) |

No CLI surface changes are required — the flags are manifest-only. No manifest version bump is required — both fields are optional and additive.

---

### 2. Manifest Model Changes

In [core/src/manifest_model.rs](core/src/manifest_model.rs), add two fields to the `Template` struct, after the existing `multi_file_output` field:

```rust
pub copy_resources_to_output: Option<bool>,
pub copy_generated_meta_content: Option<bool>,
```

Both are `Option<bool>`, defaulting to `None` (treated as `false` at every read site). No default-impl or migration changes are required.

A `manifest.toml` template entry using the new flags looks like:

```toml
[[templates]]
name        = "html"
template_type = "CustomPreprocessors"
multi_file_output = true
copy_resources_to_output    = true
copy_generated_meta_content = true

  [templates.preprocessors]
  preprocessors    = ["pandoc_html"]
  output_extension = "html"
```

---

### 3. Conversion Logic

#### 3.1 New function: `copy_resources_to_output_dir`

In [core/src/conversion.rs](core/src/conversion.rs):

```rust
fn copy_resources_to_output_dir(
    markdown_project: &MarkdownProject,
    output_dir: &Path,
    markdown_dir: &Path,
) -> Result<()> {
    for resource in markdown_project.resources.clone().unwrap_or_default() {
        let source = markdown_dir.join(&resource);

        if !source.exists() {
            return Err(eyre!(
                "Resource file {} does not exist.",
                source.display()
            ));
        }

        if source.is_dir() {
            debug!("Copying resource directory to output: {}", source.display());
            dir::copy(
                &source,
                output_dir,
                &dir::CopyOptions::new().overwrite(true).content_only(false),
            )?;
        } else {
            debug!("Copying resource file to output: {}", source.display());
            file::copy(
                &source,
                &output_dir.join(source.file_name().unwrap_or(std::ffi::OsStr::new("."))),
                &file::CopyOptions::new().overwrite(true),
            )?;
        }
    }
    Ok(())
}
```

This mirrors the existing `copy_resources` function, targeting the project output directory instead of the build directory. The duplication is intentional — the two call sites serve different purposes and the shared logic is too thin to abstract usefully.

#### 3.2 New function: `copy_generated_meta_content_to_output_dir`

```rust
fn copy_generated_meta_content_to_output_dir(
    template: &Template,
    build_dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let meta_gen = match &template.meta_gen {
        Some(m) => m,
        None => {
            warn!(
                "copy_generated_meta_content is set on template '{}' but meta_gen is not configured; skipping.",
                template.name
            );
            return Ok(());
        }
    };

    let nav_filename = meta_gen
        .nav_output
        .clone()
        .unwrap_or_else(|| PathBuf::from("nav.yml"));

    let nav_source = build_dir.join(&nav_filename);

    if nav_source.exists() {
        file::copy(
            &nav_source,
            &output_dir.join(nav_filename.file_name().unwrap_or_default()),
            &file::CopyOptions::new().overwrite(true),
        )?;
        debug!("Copied nav metadata file to output directory.");
    } else {
        warn!(
            "copy_generated_meta_content is set on template '{}' but nav file '{}' was not found in the build directory.",
            template.name,
            nav_source.display()
        );
    }

    if let Some(metadata_output) = &meta_gen.metadata_output {
        let meta_source = build_dir.join(metadata_output);
        if meta_source.exists() {
            file::copy(
                &meta_source,
                &output_dir.join(metadata_output.file_name().unwrap_or_default()),
                &file::CopyOptions::new().overwrite(true),
            )?;
            debug!("Copied metadata file to output directory.");
        }
    }

    Ok(())
}
```

A missing nav file produces a warning rather than an error: `meta_gen` is not guaranteed to produce output for every template type, and a missing file should not abort an otherwise successful conversion. The `metadata_output` file (if configured) is propagated alongside the nav file, since both are generated metadata artefacts.

#### 3.3 Call sites in the `convert` loop

In the `convert` function in [core/src/conversion.rs](core/src/conversion.rs), after `convert_template` succeeds, add:

```rust
let output_dir = project.join(markdown_project.output.clone());

if template.copy_resources_to_output.unwrap_or(false) {
    copy_resources_to_output_dir(&markdown_project, &output_dir, &input_dir)?;
}

if template.copy_generated_meta_content.unwrap_or(false) {
    copy_generated_meta_content_to_output_dir(
        &template,
        &markdown_project_compiled_directory_path,
        &output_dir,
    )?;
}
```

`template` here is the resolved `Template` struct (returned by `get_template_mapping_from_name`). `input_dir` and `markdown_project_compiled_directory_path` are already in scope within the loop. `output_dir` is constructed here and is equivalent to the path already used by `convert_template` to place the output file.

---

### 4. Integration Tests

New tests follow the pattern established in [cli/tests/convert_integration_test.rs](cli/tests/convert_integration_test.rs).

| Test name | Setup | Assertion |
|---|---|---|
| `copy_resources_to_output_copies_image_file` | Template with flag true; one image resource on markdown project | Image present in output dir after conversion |
| `copy_resources_to_output_default_false_does_not_copy` | Flag absent (default) | Image absent from output dir |
| `copy_resources_to_output_copies_directory` | Resource is a subdirectory | Subdirectory and contents present in output dir |
| `copy_generated_meta_content_propagates_nav_file` | `meta_gen` configured; `copy_generated_meta_content = true` | Nav file present in output dir |
| `copy_generated_meta_content_no_meta_gen_warns` | Flag true; `meta_gen` absent | Conversion succeeds; warning logged; no panic |
| `copy_resources_selective_by_template` | Two templates on same markdown project; flag true on HTML template only | Resource present in HTML output dir; not present alongside the other template's output |

The last test directly validates the core advantage of attaching the flags to `Template` rather than `MarkdownProject`.

---

### 5. Documentation

The two new fields should be documented in the manifest reference alongside the existing `Template` fields. The documentation should note:

- `copy_resources_to_output` reads the `resources` list from the markdown project being converted; it has no effect if `resources` is empty or absent.
- `copy_generated_meta_content` has no effect if `meta_gen` is not configured on the same template.
- Both flags are intended for web or other dynamic-asset workflows; enabling them on PDF/EPUB templates copies files that will not be used.

---

### 6. Out of Scope

- Per-resource opt-in for propagation — deferred pending a concrete use case.
- Template asset propagation (files from `template/`) — separate concern with different semantics.
- Automatic suppression of no-op copies based on detected template type — over-engineering for the current scale of the feature.
