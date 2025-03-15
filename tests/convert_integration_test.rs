use assert_cmd::Command;
use rstest::rstest;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::{TempDir, tempdir};

const VALID_MANIFEST_CONTENT_ONE_TEMPLATE: &str = r#"version = 1

[[templates]]
name = "Template for Testing"
output = "templ1.pdf"
template_file = "templ1.tex"
template_type = "Tex"
"#;

const VALID_TEMPLATE_CONTENT: &str = r#"\documentclass[a4paper,12pt]{article}

\begin{document}

\input{./output.tex}

\end{document}"#;

const VALID_MARKDOWN_CONTENT: &str = r#"# Chapter 1
Basic test content"#;

fn create_project_dir(temp_dir: &TempDir) -> PathBuf {
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    project_path
}

fn create_manifest_file(project_path: &Path, manifest_content: &str) {
    let manifest_path = project_path.join("manifest.toml");
    fs::write(manifest_path, manifest_content).expect("Failed to write manifest file");
}

fn create_template(project_path: &Path, template_name: &str, template_content: &str) {
    let template_dir = project_path.join("template");
    let template_path = template_dir.join(template_name);
    fs::create_dir_all(&template_dir).expect("Failed to create template directory");
    fs::write(template_path, template_content).expect("Failed to write template file");
}

fn create_markdown_file(project_path: &Path, filename: &str, content: &str) {
    let markdown_dir = project_path.join("Markdown");
    let markdown_path = markdown_dir.join(filename);
    fs::create_dir_all(&markdown_dir).expect("Failed to create markdown directory");
    fs::write(markdown_path, content).expect("Failed to write markdown file");
}

#[rstest]
fn test_convert() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_project_dir(&temp_dir);

    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);

    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT);

    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}
