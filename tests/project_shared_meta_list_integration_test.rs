use assert_cmd::Command;
use predicates::prelude::{PredicateBooleanExt, predicate};
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

fn add_metadata(project_path: &Path, key: &str, value: &str) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("shared-meta")
        .arg("set")
        .arg(key)
        .arg(value)
        .assert()
        .success();
}

#[rstest]
fn test_list_metadata() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    add_metadata(&project_path, "author", "John Doe");
    add_metadata(&project_path, "title", "My Title");
    add_metadata(&project_path, "date", "2025-03-27");
    add_metadata(&project_path, "description", "This is a test project");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("shared-meta")
        .arg("list")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("author: \"John Doe\"")
                .and(predicate::str::contains("title: \"My Title\""))
                .and(predicate::str::contains("date: \"2025-03-27\""))
                .and(predicate::str::contains(
                    "description: \"This is a test project\"",
                )),
        );
}

#[rstest]
fn test_list_metadata_empty() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("shared-meta")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No shared metadata fields found."));
}
