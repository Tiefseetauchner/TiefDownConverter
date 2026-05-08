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
fn test_add_profile() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("profiles")
        .arg("add")
        .arg("My funny profile")
        .arg("Template 1,Template 2")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");

    let manifest = assertions::read_manifest(&manifest_path);
    let profile = &manifest.profiles.as_ref().unwrap()[0];
    assert_eq!(profile.name, "My funny profile");
    assert_eq!(profile.templates, vec!["Template 1", "Template 2"]);
}

#[rstest]
fn test_add_profile_no_templates() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("profiles")
        .arg("add")
        .arg("My funny profile")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");

    let manifest = assertions::read_manifest(&manifest_path);
    let profile = &manifest.profiles.as_ref().unwrap()[0];
    assert_eq!(profile.name, "My funny profile");
    assert_eq!(profile.templates, Vec::<String>::new());
}
