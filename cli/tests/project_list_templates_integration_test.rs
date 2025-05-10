use assert_cmd::Command;
use predicates::prelude::*;
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

fn add_template(project_path: &Path, template_name: &str) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg(template_name)
        .arg("add")
        .arg("--template-file")
        .arg(format!("{}.tex", template_name))
        .assert()
        .success();

    let template_dir = project_path.join("template");
    assert!(template_dir.exists(), "Template directory should exist");
    fs::write(template_dir.join(format!("{}.tex", template_name)), "")
        .expect("Failed to write example template file");
}

#[rstest]
fn test_list_templates() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(&project_path, "test1");
    add_template(&project_path, "test2");
    add_template(&project_path, "test3");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("list-templates")
        .assert()
        .success()
        .stdout(
            predicate::str::contains(
                r#"test1:
  Template type: Tex
  Template file: test1.tex"#,
            )
            .and(predicate::str::contains(
                r#"test2:
  Template type: Tex
  Template file: test2.tex"#,
            ))
            .and(predicate::str::contains(
                r#"test3:
  Template type: Tex
  Template file: test3.tex"#,
            )),
        );
}

#[rstest]
fn test_list_templates_empty() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("list-templates")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}
