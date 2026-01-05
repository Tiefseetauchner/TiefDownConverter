use color_eyre::eyre::Result;
use log::debug;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{manifest_model::MetaGenerationSettings, meta_generation_format::MetaGenerationFormat};

const DEFAULT_NAV_META_YML_FILE_PATH: &str = ".meta_nav.yml";

#[derive(Serialize, Clone)]
pub struct NavMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<NavMetaNode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<NavMetaNode>,
}

#[derive(Serialize, Clone)]
pub struct NavMetaNode {
    pub id: NavMetaNodeId,
    pub path: PathBuf,
    pub title: String,
    pub prev: Option<NavMetaNodeId>,
    pub next: Option<NavMetaNodeId>,
    pub depth: usize,
}

#[derive(Serialize, Clone)]
pub struct NavMetaNodeId {
    pub value: String,
}

#[derive(Clone)]
struct PreNavNode {
    id: NavMetaNodeId,
    path: PathBuf,
    title: String,
}

pub(crate) fn retrieve_nav_meta(
    input_files: &Vec<PathBuf>,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    output_extension: &Option<String>,
) -> Result<NavMeta> {
    let canon_compiled_directory_path = &compiled_directory_path.canonicalize()?;
    let canon_conversion_input_dir = &conversion_input_dir.canonicalize()?;

    let pre_nodes: Vec<PreNavNode> = input_files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let id = format!(
                "{}_{}",
                i,
                f.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or("unknown".to_string())
            );

            let canon_file_path = canon_compiled_directory_path.join(f).canonicalize()?;
            let path = canon_file_path
                .strip_prefix(canon_conversion_input_dir)?
                .to_path_buf();

            let path = if let Some(output_extension) = output_extension {
                path.with_extension(output_extension)
            } else {
                path
            };

            let title = path.with_extension("").to_string_lossy().to_string();
            let nav_id = NavMetaNodeId { value: id };

            Ok(PreNavNode {
                id: nav_id,
                path,
                title,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let nodes = pre_nodes
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            let prev = if idx > 0 {
                Some(pre_nodes[idx - 1].id.clone())
            } else {
                None
            };
            let next = pre_nodes.get(idx + 1).map(|n| n.id.clone());
            let depth = if let Some(parent) = p.path.parent() {
                parent.iter().count()
            } else {
                0
            };

            NavMetaNode {
                id: p.id.clone(),
                path: p.path.clone(),
                title: p.title.clone(),
                prev,
                next,
                depth,
            }
        })
        .collect::<Vec<_>>();

    debug!("Built navigation metadata tree.");

    Ok(NavMeta {
        nodes: Some(nodes),
        current: None,
    })
}

pub(crate) fn generate_nav_meta_file(
    meta_gen: &MetaGenerationSettings,
    nav_meta: &NavMeta,
    compiled_directory_path: &Path,
) -> Result<PathBuf> {
    let output = meta_gen
        .nav_output
        .clone()
        .unwrap_or(PathBuf::from(DEFAULT_NAV_META_YML_FILE_PATH));

    let output = compiled_directory_path.join(output);

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let nav_meta_yaml = serde_yaml::to_string(nav_meta)?;
    fs::write(&output, nav_meta_yaml)?;
    debug!("Navigation metadata written to {}", output.display());

    if let Some(format) = meta_gen.format
        && format == MetaGenerationFormat::Json
    {
        let output = meta_gen
            .nav_output
            .clone()
            .unwrap_or(PathBuf::from(DEFAULT_NAV_META_YML_FILE_PATH))
            .with_extension("json");

        let output = compiled_directory_path.join(output);

        let nav_meta_json = serde_json::to_string(nav_meta)?;
        fs::write(&output, nav_meta_json)?;
        debug!("Navigation metadata written to {}", output.display());
    }

    Ok(output)
}
