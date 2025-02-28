use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Manifest {
    pub markdown_dir: Option<String>,
    pub templates: Vec<String>,
}
