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
        .arg("--")
        .arg("-t html")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let preprocessor = &manifest.custom_processors.preprocessors[0];
    assert_eq!(preprocessor.name, "My funny preprocessor");
    assert_eq!(preprocessor.cli_args, vec!["-t", "html"]);
}

#[rstest]
fn test_add_preprocessor_custom_cli() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("add")
        .arg("My funny preprocessor")
        .arg("--cli")
        .arg("cat")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let preprocessor = &manifest.custom_processors.preprocessors[0];
    assert_eq!(preprocessor.name, "My funny preprocessor");
    assert_eq!(preprocessor.cli.as_deref(), Some("cat"));
    assert_eq!(preprocessor.cli_args, Vec::<String>::new());
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
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let preprocessor = &manifest.custom_processors.preprocessors[0];
    assert_eq!(preprocessor.name, "My funny preprocessor");
    assert_eq!(preprocessor.cli_args, Vec::<String>::new());
}
