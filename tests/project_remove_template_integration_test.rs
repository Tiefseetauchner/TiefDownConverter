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

fn add_template(project_path: &Path, template_name: &str) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg(template_name)
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
#[case("test.tex")]
#[case("Free Text Name")]
fn test_remove_template(#[case] template_name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(&project_path, template_name);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("remove-template")
        .arg("-t")
        .arg(template_name)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");
    assert_not_contains!(
        manifest_content,
        format!(r#"name = "{}""#, template_name).as_str()
    );

    let template_dir = project_path.join("template");
    assert!(template_dir.exists(), "Template directory should exist");
    assert!(
        !template_dir.join(format!("{}.tex", template_name)).exists(),
        "Example template file should not exist"
    );
}

#[rstest]
fn test_remove_template_with_invalid_name() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(&project_path, "test.tex");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("remove-template")
        .arg("-t")
        .arg("invalid_name")
        .assert()
        .failure()
        .stderr(predicates::prelude::predicate::str::contains(
            "Template invalid_name could not be found in the project.",
        ));
}

#[rstest]
fn test_remove_template_other_templates_remain() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(&project_path, "test1");
    add_template(&project_path, "test2");
    add_template(&project_path, "test3");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("remove-template")
        .arg("-t")
        .arg("test2")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "test1"
template_type = "Tex""#
    );

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "test3"
template_type = "Tex""#
    );

    assert_not_contains!(manifest_content, r#"name = "test2""#);

    let template_dir = project_path.join("template");
    assert!(template_dir.exists(), "Template directory should exist");
    assert!(
        template_dir.join("test1.tex").exists(),
        "Example template file should exist"
    );
    assert!(
        template_dir.join("test3.tex").exists(),
        "Example template file should exist"
    );
    assert!(
        !template_dir.join("test2.tex").exists(),
        "Example template file should not exist"
    );
}
