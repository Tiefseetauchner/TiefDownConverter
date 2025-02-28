use std::error::Error;

use crate::manifest_model::Manifest;

pub fn init(project: Option<String>) -> Result<(), Box<dyn Error>> {
    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = std::path::Path::new(&project);

    if !project_path.exists() {
        std::fs::create_dir(project_path)?;
    }

    let manifest_path = project_path.join("manifest.toml");
    if manifest_path.exists() {
        return Err(
            "Manifest file already exists. Please remove it before initializing a new project."
                .into(),
        );
    }

    let manifest: Manifest = Manifest {
        markdown_dir: None,
        templates: vec!["template.tex".to_string()],
    };

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}
