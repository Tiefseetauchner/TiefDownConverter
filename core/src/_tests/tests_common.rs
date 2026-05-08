use std::path::PathBuf;

use crate::{
    consts::CURRENT_MANIFEST_VERSION,
    manifest_model::{Manifest, Processors},
    project_handle::ProjectHandle,
};

pub(crate) fn get_default_project_handle() -> ProjectHandle {
    ProjectHandle::create_with_manifest(
        PathBuf::from("."),
        Manifest {
            version: CURRENT_MANIFEST_VERSION,
            markdown_projects: None,
            templates: vec![],
            custom_processors: Processors {
                preprocessors: vec![],
                processors: vec![],
            },
            smart_clean: None,
            smart_clean_threshold: None,
            shared_metadata: None,
            metadata_settings: None,
            profiles: None,
            injections: None,
        },
    )
}
