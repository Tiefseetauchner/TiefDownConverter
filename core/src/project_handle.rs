use std::path::PathBuf;

use color_eyre::eyre::Result;

use crate::{manifest_model::Manifest, project_management::load_and_convert_manifest};

pub struct ProjectHandle {
    pub project_path: PathBuf,
    pub manifest: Manifest,
    dirty: bool,
}

impl ProjectHandle {
    pub fn open(project: Option<PathBuf>) -> Result<Self> {
        let project = project.unwrap_or(PathBuf::from("."));
        let manifest_path = project.join("manifest.toml");

        let manifest = load_and_convert_manifest(&manifest_path)?;

        Ok(ProjectHandle {
            manifest: manifest,
            project_path: project,
            dirty: false,
        })
    }

    #[cfg(test)]
    pub(crate) fn create_with_manifest(project: PathBuf, manifest: Manifest) -> Self {
        ProjectHandle {
            project_path: project,
            manifest,
            dirty: false,
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn save_if_dirty(&self) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        let manifest_content = toml::to_string(&self.manifest)?;
        std::fs::write(get_manifest_path(&self.project_path), manifest_content)?;

        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn is_dirty(&self) -> bool {
        self.dirty
    }
}

fn get_manifest_path(project_path: &PathBuf) -> PathBuf {
    project_path.join("manifest.toml")
}
