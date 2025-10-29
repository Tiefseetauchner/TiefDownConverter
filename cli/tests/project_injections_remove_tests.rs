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
fn test_injection_add(#[case] name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    create_injection(&project_path, name, vec!["a"]);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("injections")
        .arg("remove")
        .arg(name)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    let expected_manifest = format!(r#"name = "{}""#, name);

    assert_not_contains!(manifest_content, &expected_manifest);
}
