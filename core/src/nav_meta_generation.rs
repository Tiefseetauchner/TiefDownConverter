use color_eyre::eyre::Result;
use log::debug;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::manifest_model::NavMetaGenerationSettings;

pub const DEFAULT_NAV_META_FILE_PATH: &str = ".meta_nav.yml";

#[derive(Serialize, Clone)]
pub struct NavMeta {
    pub nodes: Vec<NavMetaNode>,
    pub current: Option<NavMetaNode>,
}

#[derive(Serialize, Clone)]
pub struct NavMetaNode {
    pub id: NavMetaNodeId,
    pub path: PathBuf,
    pub title: String,
    pub prev: Option<NavMetaNodeId>,
    pub next: Option<NavMetaNodeId>,
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
    let mut i: u32 = 0;

    let canon_compiled_directory_path = &compiled_directory_path.canonicalize()?;
    let canon_conversion_input_dir = &conversion_input_dir.canonicalize()?;

    let mut pre_nodes: Vec<PreNavNode> = Vec::with_capacity(input_files.len());

    for f in input_files {
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

        let title = path.to_string_lossy().to_string();
        let nav_id = NavMetaNodeId { value: id };

        pre_nodes.push(PreNavNode {
            id: nav_id,
            path,
            title,
        });

        i += 1;
    }

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

            NavMetaNode {
                id: p.id.clone(),
                path: p.path.clone(),
                title: p.title.clone(),
                prev,
                next,
            }
        })
        .collect::<Vec<_>>();

    debug!("Built navigation metadata tree.");

    Ok(NavMeta {
        nodes,
        current: None,
    })
}

pub(crate) fn generate_nav_meta_file(
    nav_meta_gen: &NavMetaGenerationSettings,
    nav_meta: &NavMeta,
    compiled_directory_path: &Path,
) -> Result<PathBuf> {
    let output = nav_meta_gen
        .output
        .clone()
        .unwrap_or(PathBuf::from(DEFAULT_NAV_META_FILE_PATH));

    let output = compiled_directory_path.join(output);

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    let nav_meta_yaml = serde_yaml::to_string(nav_meta)?;
    fs::write(&output, nav_meta_yaml)?;
    debug!("Navigation metadata written to {}", output.display());

    Ok(output.clone())
}
