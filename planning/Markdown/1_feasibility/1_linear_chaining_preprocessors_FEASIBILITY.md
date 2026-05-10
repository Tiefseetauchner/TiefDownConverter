## Linear Chaining of Preprocessors — Feasibility Study

### Abstract

TiefDownConverter's current preprocessor pipeline selects exactly one preprocessor per file-extension group and executes it as a single external process. This study assesses the feasibility of extending that pipeline to support *linear chaining*: executing an ordered sequence of preprocessors per file or file group, where the standard output of each step is fed as input to the next. The study covers architectural impact, manifest model changes, implementation complexity, testability, and practical use cases.

---

### 1. Background and Motivation

A `PreProcessor` in TiefDownConverter is a record of a CLI command (`cli`, defaulting to `pandoc`) and its arguments (`cli_args`). At conversion time, `choose_preprocessor` selects one preprocessor whose `extension_filter` matches the current file-extension chunk, and `run_preprocessor` invokes it, capturing its standard output as the chunk's result string.

This design is sufficient for single-step transformations (e.g., converting Markdown to LaTeX via pandoc). It becomes limiting when a workflow requires composing independent transformation steps — for example, normalising a source format before the primary conversion, or post-processing pandoc's output with a lightweight tool before writing it to disk. Currently, such compositions must be encoded as a single monolithic script, coupling unrelated concerns and reducing reusability.

---

### 2. Current Architecture

The relevant call chain is:

```
run_preprocessors_on_inputs
  └─ get_preprocessing_chunks          // groups files by extension
       └─ [par_iter] choose_preprocessor  // selects ONE PreProcessor per chunk
            └─ run_preprocessor           // spawns process, captures stdout
```

`run_preprocessor` uses `Stdio::piped()` for stdout capture and passes input file paths as positional CLI arguments. There is no mechanism to route the captured output back into another process.

---

### 3. Proposed Extension

#### 3.1 Conceptual Model

A *preprocessor chain* is an ordered, non-empty list of preprocessor names. Executing a chain over a set of input files means:

1. Run the first preprocessor with the original input files as arguments; capture its stdout as an intermediate string.
2. For each subsequent preprocessor, write the intermediate string to a named temporary file inside the compiled directory, pass that file as the sole argument, capture the new stdout.
3. Return the final stdout string as the chunk result.

The single-preprocessor case remains unchanged: a chain of length one degenerates to the existing behaviour.

#### 3.2 Manifest Model Changes

A new struct `PreProcessorChain` is introduced:

```toml
[[custom_processors.preprocessor_chains]]
name             = "md_to_html_clean"
extension_filter = "md"        # optional, same semantics as PreProcessor
steps            = ["pandoc_html", "html_tidy"]
```

`Processors` gains a `preprocessor_chains: Vec<PreProcessorChain>` field. A template's `PreProcessors.preprocessors` list references chain names exclusively — chains become the sole dispatch unit. Individual `PreProcessor` entries remain in `Processors.preprocessors` as building-block definitions but are no longer referenced directly by templates.

#### 3.3 Execution Changes

`run_preprocessor` is split into two functions:

- `run_preprocessor_step(step, compiled_dir, args, stdin_source)` — runs one process, either with file paths or with a temp-file path derived from `stdin_source`.
- `run_preprocessor_chain(chain, compiled_dir, metadata, input_files)` — iterates over the steps, threading the intermediate output through.

Temporary files for intermediate results are placed in the compiled directory with a deterministic name (e.g., `.chain_intermediate_{chunk_hash}.tmp`) and deleted after the chain completes, ensuring no artefacts are left behind on success. On error, they may be retained for debugging.

#### 3.4 Backward Compatibility

The manifest version is incremented. The upgrade function `upgrade_manifest_vN_to_vN+1` performs a data migration: for each `PreProcessor` defined in `custom_processors.preprocessors`, a corresponding one-step `PreProcessorChain` is created with the same name, the same `extension_filter`, and `steps` containing that preprocessor's name. Template references in `PreProcessors.preprocessors` are unchanged in value — they already hold the preprocessor name, which now matches the generated chain name. The result is transparent to existing projects: every prior single-preprocessor workflow becomes a one-step chain with identical runtime behaviour.

---

### 4. Complexity Assessment

| Concern | Assessment |
|---|---|
| Model change | Low — one new struct, one new field on `Processors` |
| Execution logic | Medium — temp-file management, error-path cleanup |
| Parallelism | No change — chains run sequentially per chunk; chunks remain parallel |
| Manifest migration | Low — data transformation required, but fully automated |
| CLI surface | None — no new subcommands; chains are defined in the manifest |
| Breaking changes | None for existing projects |

The most complex aspect is the temporary file lifecycle: files must survive long enough to be passed as CLI arguments and be cleaned up reliably even when intermediate steps fail. Rust's `Drop`-based RAII (e.g., `tempfile::NamedTempFile`) handles this cleanly.

---

### 5. Testability

#### 5.1 Unit Tests

- `choose_chain` (replacement for `choose_preprocessor`) returns the correct `PreProcessorChain` by extension filter and fallback; tested for both cases.
- `run_preprocessor_chain` tested with mock steps using `cat` and `sed` — universally available on the target platform.
- Temp-file cleanup verified by asserting file absence after successful and failed runs.

#### 5.2 Integration Tests

Integration tests follow the established pattern in `cli/tests/`. Relevant test scenarios:

1. **Two-step chain on `.md` files** — first step converts to an intermediate format, second step performs a substitution; output is compared to a known fixture.
2. **Chain with a failing intermediate step** — verifies that conversion returns an error and no partial output is written.
3. **Extension-filter interaction** — a chain is registered for `.md` and a one-step chain for `.tex`; both operate correctly in the same template.
4. **Single-element chain** — verifies behavioural equivalence with prior single-preprocessor behaviour.
5. **Manifest upgrade** — a v*N* manifest with single-preprocessor definitions is loaded; after upgrade, each preprocessor has a corresponding one-step chain and conversion produces identical output.

#### 5.3 Existing Test Impact

Existing tests that exercise conversion paths go through `run_preprocessors_on_inputs`, which now always routes through `run_preprocessor_chain`. Since the upgrade migrates every prior preprocessor to an equivalent one-step chain, the runtime behaviour is identical and no test fixture changes are required. Tests that directly call `retrieve_preprocessors` or `choose_preprocessor` by name will require signature updates.

---

### 6. Use Cases

#### 6.1 Format Normalisation Before Conversion

A project mixes Markdown files with slightly non-standard frontmatter. A first preprocessor step runs a small `sed`-based script to normalise the frontmatter syntax; the second step runs pandoc. Without chaining, the normalisation script must also invoke pandoc internally, coupling two concerns.

#### 6.2 Post-Processing pandoc Output

In a `CustomPreprocessors` template, pandoc converts Markdown to HTML. A second step runs `html-tidy` or a custom minifier. The chain ensures the final output file is clean without requiring a wrapper script.

#### 6.3 Multi-Stage Format Pipelines

A project stores content in AsciiDoc. A first step runs `asciidoctor` to produce DocBook XML; a second step runs pandoc to convert DocBook to the target format. Each tool is configured independently, and their combination is expressed declaratively in the manifest.

#### 6.4 Metadata Injection

A first preprocessor step uses a small tool to inject computed metadata (e.g., a word count or a generated timestamp) into the file as YAML frontmatter. The second step is the standard pandoc conversion. This avoids the need for a Lua filter and keeps the injection logic in a reusable, language-agnostic tool.

#### 6.5 PHP Template Rendering

A project uses `.php` source files as content templates — PHP scripts that emit Markdown when executed. A first preprocessor step invokes `php` to render the file to plain Markdown; a second step runs pandoc to convert that Markdown to the target format. This allows dynamic content (conditionals, loops, includes) to be resolved at conversion time without modifying the pandoc pipeline or embedding PHP awareness into the template system.

```toml
[[custom_processors.preprocessors]]
name     = "php_render"
cli      = "php"
cli_args = []

[[custom_processors.preprocessors]]
name     = "pandoc_md_to_html"
cli_args = ["-f", "markdown", "-t", "html"]

[[custom_processors.preprocessor_chains]]
name             = "php_to_html"
extension_filter = "php"
steps            = ["php_render", "pandoc_md_to_html"]
```

#### 6.6 Linting as a Pipeline Step

A linting or validation tool is inserted as the first step in a chain. If it exits with a non-zero code, the chain fails and conversion halts with a clear error, without the need for a separate pre-conversion hook.

---

### 7. Limitations and Risks

- **Stdin vs. file-path semantics**: Some CLI tools do not read from files passed as positional arguments and require stdin or a named flag. The temp-file approach accommodates tools that accept a file path; tools that require stdin would need an additional `stdin_mode` flag on `PreProcessorStep` to route the intermediate content via stdin rather than a temp file.
- **Debugging opacity**: A failed intermediate step produces only the error from that step; the intermediate content at the point of failure may not be visible unless the temp file is retained. A `--keep-intermediates` debug flag would improve diagnostics.
- **Performance**: Each chain step is a separate process spawn. For short-running tools over many files this adds measurable overhead. Mitigation: chains are still parallelised at the chunk level.

---

### 8. Conclusion

Linear chaining of preprocessors is technically feasible with modest implementation effort. The primary changes are confined to `manifest_model.rs` (one new struct and field) and `converters/common.rs` (chain execution logic and temp-file management). The feature introduces no breaking changes, integrates cleanly with the existing extension-filter dispatch, and substantially increases the expressiveness of the `CustomPreprocessors` conversion type. The use cases demonstrate clear, practical value for document-processing pipelines that require composing multiple independent tools.

**Recommendation: Implement.** The complexity is well-bounded, the test surface is straightforward, and the feature directly addresses a real limitation of the `CustomPreprocessors` conversion type.
