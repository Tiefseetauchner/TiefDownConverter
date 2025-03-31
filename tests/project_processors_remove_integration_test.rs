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
fn test_remove_processor() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    add_processor(
        &project_path,
        "My funny processor",
        vec!["-t", "html", "-o", "mega.html"],
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("processors")
        .arg("remove")
        .arg("My funny processor")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_not_contains!(
        manifest_content,
        r#"[[custom_processors.processors]]
name = "My funny processor"
processor_args = ["-t", "html", "-o", "mega.html"]
"#
    );
}

#[rstest]
fn test_remove_processor_does_not_exist() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("processors")
        .arg("remove")
        .arg("My funny processor")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Processor with name \'My funny processor\' does not exist.",
        ));
}

fn add_processor(project_path: &Path, processor_name: &str, processor_args: Vec<&str>) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("processors")
        .arg("add")
        .arg(processor_name)
        .arg("--")
        .arg(processor_args.join(" "))
        .assert()
        .success();
}
