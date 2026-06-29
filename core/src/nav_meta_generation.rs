use color_eyre::eyre::Result;
use log::debug;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    converters::common::run_with_logging, manifest_model::MetaGenerationSettings,
    meta_generation_format::MetaGenerationFormat,
};

const DEFAULT_NAV_META_YML_FILE_PATH: &str = ".meta_nav.yml";
const META_WRITER_FILE_NAME: &str = ".meta_writer.lua";
const META_WRITER_LUA: &str = include_str!("resources/meta_writer.lua");

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_matter: Option<serde_json::Value>,
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
    front_matter: Option<serde_json::Value>,
}

pub(crate) fn retrieve_nav_meta(
    input_files: &Vec<PathBuf>,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    output_extension: &Option<String>,
) -> Result<NavMeta> {
    let canon_compiled_directory_path = &compiled_directory_path.canonicalize()?;
    let canon_conversion_input_dir = &conversion_input_dir.canonicalize()?;

    let meta_writer_path = compiled_directory_path.join(META_WRITER_FILE_NAME);
    fs::write(&meta_writer_path, META_WRITER_LUA)?;

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

            let front_matter = read_front_matter(&canon_file_path, &meta_writer_path);

            let title = front_matter
                .as_ref()
                .and_then(|fm| fm.get("title"))
                .and_then(|t| t.as_str())
                .map(|t| t.to_string())
                .unwrap_or_else(|| path.with_extension("").to_string_lossy().to_string());

            let nav_id = NavMetaNodeId { value: id };

            Ok(PreNavNode {
                id: nav_id,
                path,
                title,
                front_matter,
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
                front_matter: p.front_matter.clone(),
            }
        })
        .collect::<Vec<_>>();

    debug!("Built navigation metadata tree.");

    Ok(NavMeta {
        nodes: Some(nodes),
        current: None,
    })
}

fn read_front_matter(file: &Path, writer_path: &Path) -> Option<serde_json::Value> {
    let mut command = Command::new("pandoc");
    command.arg(file).arg("-t").arg(writer_path);

    let stdout = match run_with_logging(command, "pandoc", true) {
        Ok(stdout) => stdout,
        Err(e) => {
            debug!(
                "Could not extract front matter from {}: {}",
                file.display(),
                e
            );
            return None;
        }
    };

    match serde_json::from_str::<serde_json::Value>(stdout.trim()) {
        Ok(serde_json::Value::Object(map)) if map.is_empty() => None,
        Ok(serde_json::Value::Array(arr)) if arr.is_empty() => None,
        Ok(serde_json::Value::Null) => None,
        Ok(value) => Some(value),
        Err(e) => {
            debug!("Could not parse front matter from {}: {}", file.display(), e);
            None
        }
    }
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
