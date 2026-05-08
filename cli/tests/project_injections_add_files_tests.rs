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
fn test_injection_add_files(#[case] file_name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    create_injection(&project_path, "injection-name", vec!["files"]);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("injections")
        .arg("add-files")
        .arg("injection-name")
        .arg(file_name)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let injection = manifest
        .injections
        .as_ref()
        .unwrap()
        .iter()
        .find(|i| i.name == "injection-name")
        .unwrap();
    assert_eq!(
        injection.files,
        vec![PathBuf::from("files"), PathBuf::from(file_name)]
    );
}

#[rstest]
#[case("injection")]
#[case("injection123")]
#[case("injection &&&")]
#[case("injection #")]
fn test_injection_add_files_preserves_order(#[case] file_name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    create_injection(&project_path, "injection-name", vec!["files"]);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("injections")
        .arg("add-files")
        .arg("injection-name")
        .arg(file_name.to_owned() + "1")
        .arg(file_name.to_owned() + "2")
        .arg(file_name.to_owned() + "3")
        .arg(file_name.to_owned() + "4")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let injection = manifest
        .injections
        .as_ref()
        .unwrap()
        .iter()
        .find(|i| i.name == "injection-name")
        .unwrap();
    assert_eq!(
        injection.files,
        vec![
            PathBuf::from("files"),
            PathBuf::from(format!("{file_name}1")),
            PathBuf::from(format!("{file_name}2")),
            PathBuf::from(format!("{file_name}3")),
            PathBuf::from(format!("{file_name}4")),
        ]
    );
}

#[rstest]
#[case("injection")]
#[case("injection123")]
#[case("injection &&&")]
#[case("injection #")]
fn test_injection_add_files_adds_duplicate(#[case] file_name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    create_injection(&project_path, "injection-name", vec![file_name]);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("injections")
        .arg("add-files")
        .arg("injection-name")
        .arg(file_name.to_owned())
        .arg(file_name.to_owned())
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let injection = manifest
        .injections
        .as_ref()
        .unwrap()
        .iter()
        .find(|i| i.name == "injection-name")
        .unwrap();
    assert_eq!(
        injection.files,
        vec![
            PathBuf::from(file_name),
            PathBuf::from(file_name),
            PathBuf::from(file_name),
        ]
    );
}
