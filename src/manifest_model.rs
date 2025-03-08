use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml::Table;

use crate::consts::CURRENT_MANIFEST_VERSION;

#[derive(Deserialize, Serialize)]
pub(crate) struct Manifest {
    pub version: u32,
    pub markdown_dir: Option<String>,
    pub templates: Vec<TemplateMapping>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct TemplateMapping {
    pub name: String,
    pub template_file: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub filters: Option<Vec<String>>,
}

pub(crate) fn upgrade_manifest(manifest: &mut Table, current_version: u32) -> Result<()> {
    if current_version != CURRENT_MANIFEST_VERSION {
        let mut updated_version = current_version;

        while updated_version < CURRENT_MANIFEST_VERSION {
            match current_version {
                0 => {
                    upgrade_manifest_v0_to_v1(manifest)?;
                }
                _ => {}
            }

            updated_version += 1;
        }
    }

    Ok(())
}

fn upgrade_manifest_v0_to_v1(manifest: &mut Table) -> Result<()> {
    manifest["version"] = toml::Value::Integer(1);

    // TODO: Convert from old template format to new template format

    Ok(())
}
