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

    fs::create_dir_all(project_path.join("template")).expect("Failed to create template directory");

    project_path
}

#[rstest]
fn test_markdown_remove() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg("main")
        .arg("Markdown")
        .arg(".")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let projects = manifest.markdown_projects.as_ref().unwrap();
    assert!(projects.iter().any(|p| p.name == "main"
        && p.path.to_str() == Some("Markdown")
        && p.output.to_str() == Some(".")));

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("remove")
        .arg("main")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let no_main = manifest
        .markdown_projects
        .as_ref()
        .map(|ps| ps.iter().all(|p| p.name != "main"))
        .unwrap_or(true);
    assert!(no_main);
}

#[rstest]
fn test_markdown_remove_does_not_exist() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("remove")
        .arg("first")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Markdown project with name 'first' does not exist.",
        ));
}
