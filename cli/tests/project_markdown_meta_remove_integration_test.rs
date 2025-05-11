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

fn add_markdown_project(project_path: &Path, name: &str) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg(name)
        .arg("Markdown")
        .arg(".")
        .assert()
        .success();
}

fn add_metadata_to_project(project_path: &Path, name: &str, key: &str, value: &str) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(project_path)
        .arg("project")
        .arg("markdown")
        .arg("meta")
        .arg(name)
        .arg("set")
        .arg(key)
        .arg(value)
        .assert()
        .success();
}

#[rstest]
fn test_markdown_meta_remove() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    add_markdown_project(&project_path, "main");

    add_metadata_to_project(&project_path, "main", "title", "My Project");
    add_metadata_to_project(&project_path, "main", "description", "A description");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("meta")
        .arg("main")
        .arg("remove")
        .arg("title")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[markdown_projects]]
name = "main"
path = "Markdown"
output = "."

[markdown_projects.metadata_fields]
description = "A description"
"#
    );

    assert_not_contains!(manifest_content, r#"title = "My Project""#);
}

#[rstest]
fn test_markdown_meta_remove_invalid_project() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("meta")
        .arg("invalid")
        .arg("remove")
        .arg("title")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Markdown project with name 'invalid' does not exist.",
        ));
}

#[rstest]
fn test_markdown_meta_remove_invalid_field() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    add_markdown_project(&project_path, "main");
    add_metadata_to_project(&project_path, "main", "title", "My Project");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("meta")
        .arg("main")
        .arg("remove")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Metadata field 'invalid' does not exist in project 'main'.",
        ));
}
