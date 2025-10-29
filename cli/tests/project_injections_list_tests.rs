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

fn create_injection(project_path: &Path, name: &str, files: Vec<&str>) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("injections")
        .arg("add")
        .arg(name)
        .arg(files.join(","))
        .assert()
        .success();
}

#[rstest]
#[case("injection")]
#[case("injection123")]
#[case("injection &&&")]
#[case("injection #")]
fn test_injection_list(#[case] name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    create_injection(&project_path, name, vec!["A", "B"]);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("injections")
        .arg("list")
        .assert()
        .success()
        .stdout(
            predicate::str::contains(name)
                .and(predicate::str::contains("  Files:"))
                .and(predicate::str::contains("    A"))
                .and(predicate::str::contains("    B")),
        );
}
