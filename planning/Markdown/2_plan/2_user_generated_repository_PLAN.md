## User Generated Repository — Implementation Plan

### Overview

Add a community-curated package registry to TiefDownConverter, backed by a public Git repository index. Users can search, pull, and publish reusable templates, Lua filters, and project scaffolds. Five new `tdc repo` subcommands expose the registry surface. No existing conversion or project-management paths are modified.

---

### 1. Registry Repository (`TiefDownConverter/tdc-registry`)

Create a separate public GitHub repository. It contains:

```
tdc-registry/
├── registry.toml          # package index
└── packages/
    └── <name>/
        └── <version>.toml # per-version descriptor (optional, for auditing)
```

#### 1.1 `registry.toml` Schema

```toml
[[packages]]
name        = "lix-novel-a4"
description = "A4 novel template using the LiX typesetting system."
author      = "tiefseetauchner"
license     = "MIT"
type        = "template"          # template | lua_filter | scaffold | bundle
trust_required = false

  [[packages.versions]]
  version  = "1.2.0"
  url      = "https://github.com/example/releases/download/v1.2.0/lix-novel-a4.zip"
  checksum = "sha256:abcdef1234..."
  requires = []                   # e.g. ["typst>=0.11", "xelatex"]
```

The index is fetched as a single file; no recursive directory traversal is required.

---

### 2. Crate Structure Changes

#### 2.1 New crate: `tdc-registry` (or module within `core`)

Given the existing separation between `core` and `cli`, registry logic belongs in `core` as a new module `core/src/registry/`:

```
core/src/registry/
├── mod.rs
├── client.rs      # RegistryClient trait + HTTP implementation
├── index.rs       # registry.toml parsing and version resolution
├── install.rs     # artifact installation per type
├── cache.rs       # local index cache management
└── publish.rs     # descriptor validation and submission preparation
```

A `RegistryClient` trait with a single real implementation (`HttpRegistryClient`) and a test double (`MockRegistryClient`) isolates network calls from all tests.

#### 2.2 `cli/src/repo_commands.rs`

New file handling the five `repo` subcommand dispatch arms, mirroring the pattern in `cli/src/project_commands.rs`.

---

### 3. CLI Commands (`cli/src/cli.rs`)

Add a `Repo` variant to the top-level `Commands` enum:

```rust
#[command(about = "Interact with the TiefDownConverter community repository.")]
Repo {
    #[arg(help = "The project to apply repository operations to. Defaults to the current directory.")]
    project: Option<PathBuf>,
    #[command(subcommand)]
    command: RepoCommands,
},
```

Add a new `RepoCommands` enum:

```rust
#[derive(Subcommand)]
pub(crate) enum RepoCommands {
    #[command(about = "Refresh the local package index cache.")]
    Update,

    #[command(about = "Search the registry for packages.")]
    Search {
        #[arg(help = "Search query (matched against name and description).")]
        query: String,
        #[arg(long, help = "Filter by artifact type (template, lua_filter, scaffold, bundle).")]
        r#type: Option<String>,
    },

    #[command(about = "Show metadata for a package.")]
    Info {
        #[arg(help = "Package name, optionally with @version (e.g. lix-novel-a4@1.2.0).")]
        package: String,
    },

    #[command(about = "Download and install a package into the current project.")]
    Pull {
        #[arg(help = "Package name, optionally with @version.")]
        package: String,
        #[arg(long, help = "For template packages, register under this template name in the manifest.")]
        template_name: Option<String>,
        #[arg(long, help = "For lua_filter packages, add the filter to this template.")]
        template: Option<String>,
        #[arg(long, help = "Overwrite existing files (required for scaffold packages on existing projects).")]
        force: bool,
    },

    #[command(about = "Prepare and submit a package to the registry.")]
    Publish {
        #[arg(help = "Path to the artifact or a package descriptor file.")]
        path: PathBuf,
        #[arg(long, help = "Package name (overrides descriptor field).")]
        name: Option<String>,
        #[arg(long, help = "Version (overrides descriptor field).")]
        version: Option<String>,
    },
}
```

---

### 4. Registry Module Implementation

#### 4.1 `cache.rs`

- Cache directory: `~/.config/tiefdownconverter/registry/` (resolved via the `dirs` crate).
- Cache file: `index.toml`.
- Staleness check: compare `mtime` of the cache file against a configurable TTL (default 86400 s). Exposed as `is_stale() -> bool`.
- `load_or_fetch(client: &dyn RegistryClient) -> Result<Index>`: returns the cached index if fresh, otherwise fetches and writes to disk.

#### 4.2 `index.rs`

```rust
pub struct Index {
    pub packages: Vec<PackageEntry>,
}

impl Index {
    pub fn from_toml(src: &str) -> Result<Self>;
    pub fn search(&self, query: &str, artifact_type: Option<&str>) -> Vec<&PackageEntry>;
    pub fn resolve(&self, name: &str, version: Option<&str>) -> Result<&VersionEntry>;
}
```

Version resolution: if `version` is `None`, select the highest semver version. Use the `semver` crate for comparison.

#### 4.3 `client.rs`

```rust
pub trait RegistryClient: Send + Sync {
    fn fetch_index(&self, url: &str) -> Result<String>;
    fn download_archive(&self, url: &str) -> Result<Vec<u8>>;
}
```

`HttpRegistryClient` uses the `ureq` crate (already a likely dependency; if not, add it — it is a blocking HTTP client with no async runtime requirement, fitting TiefDown's synchronous architecture). `MockRegistryClient` stores fixture strings and bytes in a `HashMap`.

#### 4.4 `install.rs`

```rust
pub fn install_package(
    entry: &VersionEntry,
    archive_bytes: &[u8],
    project_path: &Path,
    opts: &InstallOptions,
) -> Result<InstalledRecord>
```

`InstallOptions` carries `force`, `template_name`, `add_to_template`. The function:

1. Verifies SHA-256 checksum of `archive_bytes` against `entry.checksum`. Fails immediately on mismatch.
2. If `entry.trust_required`, prints author, description, and source URL, and prompts for confirmation via `dialoguer::Confirm`. Aborts without writing any files on denial.
3. Extracts the archive to a temp directory using the `zip` crate.
4. Delegates to the type-specific installer:
   - `install_template`: copies file(s) to `<project>/template/`, calls existing core logic to register in `manifest.toml`.
   - `install_lua_filter`: copies `.lua` files to the project directory; if `--template` provided, appends filter paths to that template's filter list in `manifest.toml`.
   - `install_scaffold`: merges the scaffold's `manifest.toml` into the project's (or overwrites with `--force`), copies auxiliary files.
   - `install_bundle`: iterates the bundle's component descriptors and calls the relevant sub-installer for each.
5. Appends an `[[repository.installed]]` entry to the project's `manifest.toml`.
6. Cleans the temp directory.

On any error after step 2 but before step 6, previously written files are rolled back (tracked by a `Vec<PathBuf>` of written paths, cleaned via `fs::remove_file` in the error path).

#### 4.5 `publish.rs`

`publish` is a submission helper, not an upload command (in the Git-backed model):

1. Validates the package descriptor TOML/JSON against the schema (`serde` deserialization with `deny_unknown_fields`).
2. Computes the archive checksum.
3. Prints the complete descriptor that would be added to `registry.toml`.
4. Opens the browser to a pre-filled GitHub new-file URL targeting the `tdc-registry` repository, with the descriptor pre-populated in the URL's `value` parameter (GitHub supports this for files under ~2 KB).

---

### 5. Manifest Model (`core/src/manifest_model.rs`)

Add an optional `repository` table. No version bump is needed; the field uses `#[serde(default)]` and is silently ignored by older versions:

```rust
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct RepositorySettings {
    #[serde(default)]
    pub registry_url: Option<String>,
    #[serde(default)]
    pub installed: Vec<InstalledRecord>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct InstalledRecord {
    pub name: String,
    pub version: String,
    pub r#type: String,
}
```

`Manifest` gains `#[serde(default)] pub repository: RepositorySettings`.

---

### 6. Global Configuration

A global configuration file at `~/.config/tiefdownconverter/config.toml` stores the canonical registry URL and TTL:

```toml
registry_url = "https://raw.githubusercontent.com/TiefDownConverter/tdc-registry/main/registry.toml"
cache_ttl_seconds = 86400
```

Resolved via a new `core/src/global_config.rs` module. If the file does not exist, built-in defaults are used.

---

### 7. Dependencies

Add to `core/Cargo.toml`:

```toml
ureq        = "2"         # blocking HTTP client
zip         = "2"         # archive extraction
sha2        = "0.10"      # checksum verification
semver      = "1"         # version resolution
dirs        = "5"         # platform-appropriate config/cache paths
dialoguer   = "0.11"      # interactive trust confirmation prompt
```

---

### 8. Test Plan

#### 8.1 Unit Tests (`core/src/_tests/registry_tests.rs`)

| Test | Assertion |
|---|---|
| `index_parses_valid_toml` | A fixture `registry.toml` deserializes without error |
| `index_search_by_name` | Query matching one of three fixture packages returns exactly that package |
| `index_search_by_type` | `type = "lua_filter"` filter returns only matching entries |
| `resolve_latest_version` | No explicit version → highest semver version selected |
| `resolve_exact_version` | `@1.0.0` resolves to the correct entry |
| `resolve_unknown_package` | Returns `Err` |
| `checksum_mismatch_aborts` | `install_package` with a tampered byte slice returns `Err` before writing any files |
| `install_template_copies_files` | Fixture zip with a single `.tex` file lands in `template/`; `manifest.toml` is updated |
| `install_lua_filter_appends_to_template` | Fixture zip with a `.lua` file appended to a named template's filters |
| `install_scaffold_respects_force` | Without `--force`, scaffold on an existing project returns `Err`; with `--force`, succeeds |
| `install_rollback_on_failure` | Simulated failure mid-install; no files remain in the project |
| `cache_staleness_check` | Mock mtime older than TTL → `is_stale()` returns `true` |
| `trust_prompt_denial_aborts` | Mock `Confirm` returning `false` → `install_package` returns `Err` without writing files |

#### 8.2 Integration Tests (`cli/tests/repo_integration_tests.rs`)

| Test | Scenario |
|---|---|
| `repo_pull_template_end_to_end` | Fresh project; `pull` with mock client and fixture archive; template listed in `manifest.toml` |
| `repo_pull_checksum_failure` | Mock client returns archive with mismatched checksum; command exits non-zero |
| `repo_search_prints_results` | Fixture index with two packages; `search` output contains expected names |
| `repo_info_prints_metadata` | `info lix-novel-a4@1.2.0` prints author, description, version, checksum |
| `repo_pull_bundle_installs_all_components` | Fixture bundle with one template and one filter; both are installed and registered |
| `repo_update_writes_cache` | `update` with mock client; cache file exists and contains fixture index after command |

---

### 9. Documentation

Add a "Repository" section to `docs/` covering:

- `tdc repo` command reference.
- How to create and publish a package (descriptor schema, archive layout per artifact type).
- How to self-host a private registry (pointing `registry_url` to a custom URL).

---

### 10. Delivery Sequence

1. Registry repository: create `tdc-registry` on GitHub with initial `registry.toml` (one or two seed packages).
2. `core`: add `global_config.rs`; add `registry/` module skeleton with trait definitions and `index.rs` parsing.
3. `core`: implement `cache.rs` and `client.rs`; unit tests (red → green).
4. `core`: implement `install.rs` for the `template` artifact type only; unit tests (red → green).
5. `cli`: add `RepoCommands` to `cli.rs`; add `repo_commands.rs` with `update`, `search`, `info`, `pull` (template only) dispatch; integration tests for those four commands (red → green).
6. `core`: extend `install.rs` for `lua_filter` and `scaffold` artifact types; unit tests (red → green); extend integration tests.
7. `core`: implement `publish.rs`; add `tdc repo publish` CLI arm; unit test descriptor validation.
8. `core`: extend `install.rs` for `bundle` type; unit and integration tests (red → green).
9. Documentation update.
