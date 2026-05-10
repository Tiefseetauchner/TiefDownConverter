## Linear Chaining of Preprocessors — Implementation Plan

### Overview

Extend the preprocessor pipeline to support ordered, linear chains of preprocessors. The output of each step feeds as input to the next, enabling multi-tool workflows without requiring wrapper scripts. Chains become the sole dispatch unit; existing single-preprocessor configurations are migrated automatically to one-step chains during manifest upgrade.

---

### 1. Manifest Model (`core/src/manifest_model.rs`)

#### 1.1 Add `PreProcessorChain`

```rust
/// An ordered sequence of preprocessor steps executed in series.
///
/// The stdout of step N is written to a temporary file and passed as input to step N+1.
#[derive(Deserialize, Serialize, Clone)]
pub struct PreProcessorChain {
    pub name: String,
    pub extension_filter: Option<String>,
    pub steps: Vec<String>,
}
```

#### 1.2 Extend `Processors`

Add the new field to `Processors`:

```rust
pub struct Processors {
    pub preprocessors: Vec<PreProcessor>,
    pub preprocessor_chains: Vec<PreProcessorChain>,
    pub processors: Vec<Processor>,
}
```

Default the field to an empty vec via `#[serde(default)]`.

#### 1.3 Manifest Version Bump

Increment `CURRENT_MANIFEST_VERSION` in `core/src/consts.rs`. Add `upgrade_manifest_vN_to_vN+1` that iterates over every entry in `custom_processors.preprocessors` and creates a corresponding `PreProcessorChain` with the same `name`, the same `extension_filter`, and `steps = [preprocessor_name]`. The chain name matches the preprocessor name, so existing `PreProcessors.preprocessors` references in templates resolve correctly without modification. Register the upgrade in `upgrade_manifest`.

---

### 2. Resolution Layer (`core/src/converters/common.rs`)

#### 2.1 Update `retrieve_preprocessors`

Change the return type from `Vec<PreProcessor>` to `Vec<PreProcessorChain>`. The function resolves each name in `PreProcessors.preprocessors` against `custom_processors.preprocessor_chains`. Update all call sites accordingly.

#### 2.2 Replace `choose_preprocessor` with `choose_chain`

```rust
fn choose_chain(chains: &[PreProcessorChain], extension: &str) -> Result<&PreProcessorChain>
```

Selection logic is identical to the current `choose_preprocessor`: prefer the entry whose `extension_filter` glob-matches `extension`; fall back to the entry with no filter. No dispatch branching is needed — the return type is always `PreProcessorChain`.

---

### 3. Execution Layer (`core/src/converters/common.rs`)

#### 3.1 `run_preprocessor_chain`

```rust
fn run_preprocessor_chain(
    template: &Template,
    compiled_directory_path: &Path,
    metadata_fields: &Table,
    metadata_file: &Option<PathBuf>,
    nav_meta_data: &Option<(NavMeta, PathBuf)>,
    steps: &[PreProcessor],
    files: &[PathBuf],
) -> Result<String>
```

Algorithm:

1. Run `run_preprocessor` with the original `files` for `steps[0]`; capture result string.
2. For each subsequent step:
   a. Write the intermediate string to `compiled_directory_path/.chain_intermediate_{step_index}.tmp` using `tempfile::NamedTempFile` (kept on disk via `keep()`).
   b. Call `run_preprocessor` with the temp file path as the sole entry in `files`.
   c. Delete the temp file.
3. Return the final result string.

Error handling: if any step fails, propagate the error immediately. Temp files created before the failure are cleaned up in the error path.

#### 3.2 Update `run_preprocessors_on_inputs`

Replace the `choose_preprocessor` + `run_preprocessor` call with `choose_chain` + `run_preprocessor_chain`. No dispatch branching is needed:

```rust
let chain = choose_chain(&chains, &chunk.1)?;
run_preprocessor_chain(template, compiled_directory_path, ..., chain, &chunk.0)
```

The parallel `par_iter()` over chunks is unchanged. `run_preprocessor` becomes a private helper called only from within `run_preprocessor_chain`.

---

### 4. Dependency

Add `tempfile` to `core/Cargo.toml` if not already present:

```toml
[dependencies]
tempfile = "3"
```

Alternatively, construct temp file paths manually using a deterministic name derived from the thread ID or chunk index, and delete them explicitly. The `tempfile` crate is preferable for correctness on panic paths.

---

### 5. Test Plan

#### 5.1 Unit Tests (`core/src/_tests/`)

File: `core/src/_tests/preprocessor_chain_tests.rs`

| Test | Assertion |
|---|---|
| `retrieve_chains_filters_by_name` | Only chains whose names appear in `PreProcessors.preprocessors` are returned |
| `retrieve_chains_unknown_name` | Returns an error when a referenced name has no matching chain |
| `choose_chain_by_extension` | Extension-filtered chain is preferred over no-filter chain |
| `choose_chain_fallback` | No-filter chain selected when no filter matches the extension |
| `choose_chain_no_match` | Returns an error when no chain matches |
| `run_chain_two_steps` | `cat` followed by `sed 's/a/b/'` on a fixture file produces the expected output |
| `run_chain_intermediate_cleanup` | After a successful chain, no `.chain_intermediate_*.tmp` files remain in the compiled dir |
| `run_chain_step_failure` | A failing intermediate step causes `run_preprocessor_chain` to return `Err` |
| `run_chain_single_step` | A one-step chain produces the same output as a direct `run_preprocessor` call |

#### 5.2 Integration Tests (`cli/tests/`)

File: `cli/tests/convert_preprocessor_chain_integration_test.rs`

| Test | Scenario |
|---|---|
| `chain_two_step_md_conversion` | Template with a two-step chain; fixture Markdown processed through `cat` + `sed`; output matches expected fixture |
| `chain_failing_intermediate` | Second step exits non-zero; `tdc convert` returns an error exit code |
| `chain_extension_filter_coexistence` | Template uses a two-step chain for `.md` and a one-step chain for `.tex`; both produce correct output |
| `chain_single_element` | One-step chain produces identical output to the pre-migration single preprocessor behaviour |
| `manifest_upgrade_migrates_preprocessors` | Old manifest with single-preprocessor definitions loaded; after upgrade, each preprocessor has a matching one-step chain; conversion output is unchanged |

---

### 6. Documentation

Update `planning/Markdown/` with usage examples showing the new TOML syntax. Update any existing documentation that describes the `custom_processors` table to mention `preprocessor_chains`.

---

### 7. Delivery Sequence

1. Manifest model: add `PreProcessorChain`, extend `Processors`, bump version, add upgrade function with data migration.
2. Update `retrieve_preprocessors` return type; update all call sites to compile.
3. Replace `choose_preprocessor` with `choose_chain`.
4. Add `run_preprocessor_chain`; update `run_preprocessors_on_inputs` to call it exclusively.
5. Unit tests (red → green).
6. Integration tests (red → green).
7. Documentation update.
