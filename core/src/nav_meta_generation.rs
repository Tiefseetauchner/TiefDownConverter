use color_eyre::eyre::Result;
use std::path::{Path, PathBuf};

/// Navigation metadata
///
///
pub struct NavMeta {
    pub nodes: Vec<NavMetaNode>,
}

pub struct NavMetaNode {
    pub id: NavMetaNodeId,
    pub path: PathBuf,
    pub title: String,
    pub prev: Option<NavMetaNodeId>,
    pub next: Option<NavMetaNodeId>,
    pub parent: Option<NavMetaNodeId>,
    pub children: Vec<NavMetaNodeId>,
}

pub struct NavMetaNodeId {
    pub value: String,
}

pub(crate) fn retrieve_nav_meta(
    input_files: &Vec<PathBuf>,
    project_directory_path: &Path,
    _compiled_directory_path: &Path,
    conversion_input_dir: &Path,
) -> Result<NavMeta> {
    let mut i: u32 = 0;

    // let canon_compiled_directory_path = &compiled_directory_path.canonicalize()?;
    let canon_conversion_input_dir = &conversion_input_dir.canonicalize()?;

    let nodes = input_files
        .iter()
        .map(|f| {
            let id = format!(
                "{}_{}",
                i,
                f.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or("unknown".to_string())
            );

            let canon_file_path = project_directory_path.join(f).canonicalize()?;
            let path = canon_file_path
                .strip_prefix(canon_conversion_input_dir)?
                .to_path_buf();

            let title = path.to_string_lossy().to_string();

            let node = NavMetaNode {
                id: NavMetaNodeId { value: id },
                path,
                title,
                prev: None,
                next: None,
                parent: None,
                children: vec![],
            };

            i += 1;

            Ok(node)
        })
        .collect::<Result<Vec<NavMetaNode>>>()?;

    Ok(NavMeta { nodes })
}
