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

    let manifest = assertions::read_manifest(&manifest_path);
    let profiles = manifest.profiles.as_ref().unwrap();
    assert!(
        profiles.iter().any(
            |p| p.name == "My funny profile" && p.templates == vec!["Template 1", "Template 2"]
        )
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("profiles")
        .arg("remove")
        .arg("My funny profile")
        .assert()
        .success();

    let manifest = assertions::read_manifest(&manifest_path);
    let has_profile = manifest
        .profiles
        .as_ref()
        .map(|ps| ps.iter().any(|p| p.name == "My funny profile"))
        .unwrap_or(false);
    assert!(!has_profile);
}

#[rstest]
fn test_remove_profile_does_not_exist() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");

    cmd.current_dir(&project_path)
        .arg("project")
        .arg("profiles")
        .arg("remove")
        .arg("Non-existent profile")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Profile with name \'Non-existent profile\' does not exist.",
        ));
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
