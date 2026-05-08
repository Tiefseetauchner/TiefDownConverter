use assert_cmd::Command;
use predicates::prelude::predicate;
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

fn add_preprocessor(project_path: &Path, preprocessor_name: &str) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("add")
        .arg(preprocessor_name)
        .arg("--")
        .arg("--listings")
        .assert()
        .success();
}

#[rstest]
fn test_remove_preprocessor() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    add_preprocessor(&project_path, "My funny preprocessor");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("remove")
        .arg("My funny preprocessor")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    assert!(
        !manifest
            .custom_processors
            .preprocessors
            .iter()
            .any(|p| p.name == "My funny preprocessor")
    );
}

#[rstest]
fn test_remove_preprocessor_others_remain() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    add_preprocessor(&project_path, "My funny preprocessor");
    add_preprocessor(&project_path, "My best preprocessor");
    add_preprocessor(&project_path, "Remove this");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("remove")
        .arg("Remove this")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    assert!(
        !manifest
            .custom_processors
            .preprocessors
            .iter()
            .any(|p| p.name == "Remove this")
    );
    assert!(
        manifest
            .custom_processors
            .preprocessors
            .iter()
            .any(|p| p.name == "My funny preprocessor" && p.cli_args == vec!["--listings"])
    );
    assert!(
        manifest
            .custom_processors
            .preprocessors
            .iter()
            .any(|p| p.name == "My best preprocessor" && p.cli_args == vec!["--listings"])
    );
}

#[rstest]
fn test_remove_preprocessor_does_not_exist() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    add_preprocessor(&project_path, "My funny preprocessor");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("remove")
        .arg("Remove this")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Preprocessor with name 'Remove this' does not exist.",
        ));

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    assert!(
        manifest
            .custom_processors
            .preprocessors
            .iter()
            .any(|p| p.name == "My funny preprocessor" && p.cli_args == vec!["--listings"])
    );
}
