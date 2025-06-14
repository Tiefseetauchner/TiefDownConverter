use assert_cmd::Command;
use rstest::rstest;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::tempdir;

#[path = "assertions.rs"]
#[macro_use]
mod assertions;

fn create_empty_project(temp_dir: &Path) -> PathBuf {
    let project_path = temp_dir.join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("-n")
        .assert()
        .success();

    project_path
}

#[rstest]
fn test_add_preprocessor() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("add")
        .arg("My funny preprocessor")
        .arg("mega.html")
        .arg("--")
        .arg("-t html")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[custom_processors.preprocessors]]
name = "My funny preprocessor"
pandoc_args = ["-t", "html"]
combined_output = "mega.html""#
    );
}

#[rstest]
fn test_add_preprocessor_no_args() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("add")
        .arg("My funny preprocessor")
        .arg("output.tex")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[custom_processors.preprocessors]]
name = "My funny preprocessor"
pandoc_args = []
combined_output = "output.tex""#
    );
}
