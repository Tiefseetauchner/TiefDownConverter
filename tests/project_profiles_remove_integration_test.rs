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
fn test_remove_profile() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    add_profile(
        &project_path,
        "My funny profile",
        vec!["Template 1", "Template 2"],
    );

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");

    let manifest_content =
        fs::read_to_string(&manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[profiles]]
name = "My funny profile"
templates = ["Template 1", "Template 2"]
"#
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("profiles")
        .arg("remove")
        .arg("My funny profile")
        .assert()
        .success();

    let manifest_content =
        fs::read_to_string(&manifest_path).expect("Failed to read manifest file");

    assert_not_contains!(
        manifest_content,
        r#"[[profiles]]
name = "My funny profile"
templates = ["Template 1", "Template 2"]
"#
    );
}

fn add_profile(project_path: &Path, profile_name: &str, templates: Vec<&str>) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("profiles")
        .arg("add")
        .arg(profile_name)
        .arg(templates.join(","))
        .assert()
        .success();
}
