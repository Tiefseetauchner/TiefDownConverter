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

fn create_markdown_project(name: &str, input: &str, output: &str, project_path: &Path) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg(&name)
        .arg(&input)
        .arg(&output)
        .assert()
        .success();
}

#[rstest]
fn test_markdown_resources_remove() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    create_markdown_project("name", "input", "output", &project_path);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("resources")
        .arg("name")
        .arg("add")
        .arg("--")
        .arg("path1")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let project = manifest
        .markdown_projects
        .as_ref()
        .unwrap()
        .iter()
        .find(|p| p.name == "name")
        .unwrap();
    assert_eq!(
        project.resources.as_ref().unwrap(),
        &vec![PathBuf::from("path1")]
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("resources")
        .arg("name")
        .arg("remove")
        .arg("path1")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let project = manifest
        .markdown_projects
        .as_ref()
        .unwrap()
        .iter()
        .find(|p| p.name == "name")
        .unwrap();
    let has_path1 = project
        .resources
        .as_ref()
        .map(|r| r.iter().any(|p| p.to_str() == Some("path1")))
        .unwrap_or(false);
    assert!(!has_path1);
}

#[rstest]
fn test_markdown_resources_remove_leaves_others() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    create_markdown_project("name", "input", "output", &project_path);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("resources")
        .arg("name")
        .arg("add")
        .arg("--")
        .arg("path1")
        .arg("path2")
        .arg("path3")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let project = manifest
        .markdown_projects
        .as_ref()
        .unwrap()
        .iter()
        .find(|p| p.name == "name")
        .unwrap();
    assert_eq!(
        project.resources.as_ref().unwrap(),
        &vec![
            PathBuf::from("path1"),
            PathBuf::from("path2"),
            PathBuf::from("path3")
        ]
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("resources")
        .arg("name")
        .arg("remove")
        .arg("path1")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let project = manifest
        .markdown_projects
        .as_ref()
        .unwrap()
        .iter()
        .find(|p| p.name == "name")
        .unwrap();
    let resources = project.resources.as_ref().unwrap();
    assert!(!resources.iter().any(|p| p.to_str() == Some("path1")));
    assert_eq!(
        resources,
        &vec![PathBuf::from("path2"), PathBuf::from("path3")]
    );
}
