## User Generated Repository — Feasibility Study

### Abstract

TiefDownConverter currently ships a fixed set of built-in preset templates and relies on manual file management for any user-defined content (templates, Lua filters, project structures). This study assesses the feasibility of a *User Generated Repository* (UGR): a hosted, community-curated registry from which users can pull reusable content into their projects and to which they can publish their own. The study covers repository architecture options, content classification, authentication and trust, manifest integration, CLI surface, testability, and practical use cases.

---

### 1. Background and Motivation

TiefDownConverter's extensibility model is entirely local. A user who creates a sophisticated LaTeX template for novel typesetting, a Lua filter for cross-reference normalisation, or a complete project scaffold for academic papers has no mechanism to share that work with the broader TiefDown community beyond copying files manually. Conversely, a new user bootstrapping a project must either start from the sparse set of built-in presets or discover external files through informal channels.

A community repository addresses both gaps: it lowers the cost of sharing finished artifacts and lowers the cost of starting a new project by providing a curated, discoverable pool of reusable content.

---

### 2. Content Classification

The repository must accommodate several distinct artifact types, each with different structures and consumption semantics:

| Artifact Type | Description | Consumed By |
|---|---|---|
| **Template** | A single `.tex`, `.typ`, or EPUB template directory | `tdc project templates add` |
| **Lua Filter** | One or more `.lua` files, optionally with a metadata header | `tdc project templates update --add-filters` |
| **Project Scaffold** | A full `manifest.toml` skeleton and optional directory structure | `tdc init` |
| **Bundle** | A named collection combining templates, filters, and scaffolding | Any of the above |

Each artifact is identified by a *package name* (e.g., `lix-novel-a4`) and a *semantic version*. A package descriptor (a small TOML or JSON document) accompanies each artifact, declaring its name, version, type, author, description, license, and dependencies on external tools (e.g., requires `typst >= 0.11`).

---

### 3. Repository Architecture Options

#### 3.1 Git-Backed Index (Recommended)

A dedicated public Git repository (e.g., `TiefDownConverter/tdc-registry`) holds:

- An index file (`registry.toml` or `registry.json`) mapping package names to versions and download URLs.
- One entry per package, pointing to a versioned archive hosted as a GitHub Release asset or an external URL supplied by the author.

The client fetches only the index on `tdc repo update`, resolves packages locally, and downloads individual archives on demand. This approach requires no custom backend infrastructure, is version-controlled by default, and leverages GitHub's CDN for distribution.

#### 3.2 Dedicated REST API

A lightweight API server (e.g., written with Axum) exposes endpoints for search, metadata lookup, and archive download. Publishing requires authenticated POST requests. This provides richer query capabilities (full-text search, filtering by artifact type) but introduces infrastructure maintenance overhead and operational costs.

#### 3.3 Recommendation

Begin with the Git-backed index. It requires no server infrastructure, is immediately auditable, and is sufficient for the expected scale of a focused document-tooling community. A migration path to a REST API exists if the index approach proves limiting; the CLI abstraction layer (see §5) would hide this transition from users.

---

### 4. Authentication and Trust

#### 4.1 Publishing

Publishing is modelled on the crates.io contributor model: a contributor forks the registry repository, adds or updates their package entry, and opens a pull request. Maintainers review and merge. This makes every published package a conscious curatorial decision and avoids the need for a bespoke authentication server.

Automated publishing (e.g., from a CI pipeline) can be accommodated by a GitHub Actions workflow that validates the package descriptor, runs a schema check, and auto-merges if validation passes and the package version is new. The contributor's GitHub identity provides the authentication anchor.

#### 4.2 Trust and Safety

Downloaded archives are verified against a SHA-256 checksum recorded in the index entry. On installation, `tdc` computes the checksum of the downloaded file and aborts if it does not match. This prevents corruption in transit and detects tampered hosted files.

Lua filters execute arbitrary code during pandoc conversion. The package descriptor must carry a `trust_required = true` flag for any artifact containing executable content. On first installation of such an artifact, the CLI prompts the user for explicit confirmation and displays the package's author, description, and source URL before proceeding.

---

### 5. CLI Surface

Five new subcommands are proposed, grouped under a `repo` top-level command:

```
tdc repo update                          # Refresh the local index cache
tdc repo search <query>                  # Search by name or description
tdc repo info <package>[@<version>]      # Show package metadata
tdc repo pull <package>[@<version>]      # Download and install into the current project
tdc repo publish <path>                  # Publish a local artifact to the registry
```

#### 5.1 `tdc repo update`

Fetches the current `registry.toml` from the canonical registry URL (configurable via a global TiefDown settings file) and stores it in the local cache directory (`~/.config/tiefdownconverter/registry/`). Called automatically before `search` and `info` if the cache is stale (older than a configurable TTL, defaulting to 24 hours).

#### 5.2 `tdc repo pull`

Resolves the requested package against the local cache, downloads the archive, verifies its checksum, and installs it according to its declared artifact type:

- **Template**: copies the template file(s) into `template/` and calls the existing `add-template` logic to register the entry in `manifest.toml`.
- **Lua Filter**: copies the filter file(s) into the project directory and optionally appends the filter to a specified template via `--template`.
- **Project Scaffold**: unpacks the scaffold into the project root; may optionally overwrite `manifest.toml` (requires `--force`).
- **Bundle**: processes each component in declaration order.

The installed package and its version are recorded in a `[repository]` section of `manifest.toml`, enabling `tdc repo update-packages` to check for newer versions in a future iteration.

#### 5.3 `tdc repo publish`

Validates the local artifact against the package descriptor schema, computes the archive checksum, and guides the user through the submission process. In the Git-backed model, `publish` opens the browser to a pre-filled GitHub pull-request URL. In a future REST model, it would POST the archive and descriptor directly.

---

### 6. Manifest Integration

A new optional `[repository]` table records installed packages:

```toml
[repository]
registry_url = "https://raw.githubusercontent.com/TiefDownConverter/tdc-registry/main/registry.toml"

[[repository.installed]]
name    = "lix-novel-a4"
version = "1.2.0"
type    = "template"

[[repository.installed]]
name    = "smart-quotes"
version = "0.3.1"
type    = "lua_filter"
```

This table is optional and ignored by conversion commands; it exists solely as a record for tooling and future update-check functionality. No manifest version bump is required because the field is unknown to older versions and will be silently dropped on upgrade.

---

### 7. Complexity Assessment

| Concern | Assessment |
|---|---|
| Registry infrastructure | Low (Git-backed); Medium (REST API) |
| CLI additions | Medium — five new subcommands, HTTP client, cache management |
| Archive handling | Low — zip/tar extraction; well-supported in Rust ecosystem |
| Checksum verification | Low — `sha2` crate |
| Trust prompting | Low — one interactive confirmation gate |
| Manifest changes | Low — additive, optional table; no migration required |
| Cross-platform install paths | Medium — path handling differs between Windows and Unix |
| Testing | Medium — network calls require mocking or fixture servers |

The most complex aspect is the install logic for bundles, which must compose multiple artifact-type-specific installation paths without leaving the project in a partially-installed state on failure.

---

### 8. Testability

#### 8.1 Unit Tests

- Package descriptor deserialization: valid and invalid TOML/JSON fixtures.
- Index parsing: version resolution, checksum field presence.
- Checksum verification: known-good and tampered archives.
- Install path resolution per artifact type and platform.
- Cache staleness check: mock system time.

#### 8.2 Integration Tests

The HTTP client is abstracted behind a `RegistryClient` trait with a real implementation and a test double. Integration tests use the test double, providing a fixture registry index and pre-built fixture archives:

| Test | Scenario |
|---|---|
| `pull_template_registers_in_manifest` | `pull` installs a fixture template; `manifest.toml` lists it correctly |
| `pull_with_checksum_mismatch_aborts` | Tampered archive is rejected before any files are written |
| `pull_lua_filter_prompts_trust` | A `trust_required` filter prompts the user; denial aborts without installation |
| `pull_scaffold_populates_project` | A scaffold archive unpacks into the correct directory layout |
| `search_returns_matching_packages` | A fixture index with three entries; query matches one by name |
| `publish_validates_descriptor` | A malformed descriptor is rejected before submission |

#### 8.3 Existing Test Impact

No existing tests are affected. The repository commands are entirely additive; conversion, project management, and initialization paths are unchanged.

---

### 9. Use Cases

#### 9.1 Novel Author Bootstrap

A user starting a new novel project runs:

```sh
tdc init --no-templates
tdc repo pull lix-novel-a4
tdc repo pull lix-novel-book
tdc repo pull chapter-heading-filter
```

Within three commands, the project is ready for conversion with professionally typeset chapter headings, without any manual template file management.

#### 9.2 Academic Paper Template Sharing

A research group maintains a shared LaTeX template for their journal submissions. They publish it to the registry once. All group members run `tdc repo pull group-journal-template` to receive the same template, with version-locked reproducibility.

#### 9.3 Lua Filter Ecosystem

Short, reusable Lua filters (smart-quotes normalisation, figure caption formatting, footnote rewriting) are published individually. Users assemble a per-project filter stack from these building blocks without duplicating filter code across repositories.

#### 9.4 Project Scaffold for Technical Documentation

A software team maintains a scaffold that includes a `manifest.toml` with a documentation-specific template, a Lua filter for code block styling, and a shared metadata file with company-standard fields. New documentation projects are bootstrapped by:

```sh
tdc init --no-templates
tdc repo pull company-docs-scaffold --force
```

#### 9.5 Typst Template Distribution

A designer publishes a visually polished Typst template. Because Typst templates do not require external LaTeX installations, the package can be adopted by any TiefDown user regardless of their LaTeX environment, widening the potential contributor and consumer base.

---

### 10. Limitations and Risks

- **Content quality**: The Git-backed pull-request model provides a curation gate but scales poorly if submission volume grows. A `categories` and `quality` tagging system would help discoverability without requiring deeper infrastructure.
- **Registry centralisation**: A single canonical registry is a single point of failure. An optional `registry_url` override in the manifest and a global config file mitigates this; teams can self-host a private registry.
- **Executable artifacts**: Lua filters execute arbitrary code. The trust-prompt gate is a usability friction point; users may habituate to confirming without reading. Clear, persistent provenance display (author, source URL, commit hash in the index) is the primary mitigation.
- **Windows path handling**: Archive extraction and install-path construction must account for Windows path separators and reserved filenames. This is a known source of subtle bugs and requires explicit test coverage on Windows runners.
- **Versioning strategy for project scaffolds**: Scaffolds do not naturally version the same way libraries do. Pulling a newer version of a scaffold into an existing project risks overwriting user edits. The safest default is to refuse overwriting and require `--force`.

---

### 11. Conclusion

A User Generated Repository is technically feasible with moderate implementation effort. The Git-backed index approach eliminates the need for custom server infrastructure while providing a fully auditable, version-controlled package history. The five proposed CLI subcommands integrate cleanly with the existing command structure. The primary engineering risks — cross-platform install paths and the trust surface of executable Lua artifacts — are well-understood and have established mitigations.

**Recommendation: Implement, starting with the Git-backed index and `pull`/`search`/`info` commands.** Publishing via pull-request can ship in the same iteration. An automated publishing API and update-check functionality are natural second-iteration additions once the registry has established adoption.
