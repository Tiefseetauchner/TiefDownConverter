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
fn test_markdown_meta_list() {
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
        .arg("list")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("title: \"My Project\"")
                .and(predicate::str::contains("description: \"A description\"")),
        );
}

#[rstest]
fn test_markdown_meta_list_no_metadata() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    add_markdown_project(&project_path, "main");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("meta")
        .arg("main")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No metadata fields found."));
}
