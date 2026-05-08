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
fn test_markdown_add() {
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
}

#[rstest]
fn test_markdown_add_with_existing_project() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg("first")
        .arg("Markdown1")
        .arg("out1")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg("second")
        .arg("Markdown2")
        .arg("out2")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let projects = manifest.markdown_projects.as_ref().unwrap();
    assert!(projects.iter().any(|p| p.name == "first"
        && p.path.to_str() == Some("Markdown1")
        && p.output.to_str() == Some("out1")));
    assert!(projects.iter().any(|p| p.name == "second"
        && p.path.to_str() == Some("Markdown2")
        && p.output.to_str() == Some("out2")));
}

#[rstest]
fn test_markdown_add_fails_to_overwrite() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg("first")
        .arg("Markdown1")
        .arg("out1")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg("first")
        .arg("Markdown2")
        .arg("out2")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Markdown project with name 'first' already exists.",
        ));
}
