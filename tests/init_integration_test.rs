use assert_cmd::Command;
use rstest::rstest;
use std::{fs, path::Path};
use tempfile::tempdir;

#[path = "assertions.rs"]
#[macro_use]
mod assertions;

const DEFAULT_MANIFEST_CONTENT: &str = r#"version = 2

[[templates]]
name = "template.tex"
template_type = "Tex"

[custom_processors]
preprocessors = []
"#;

fn assert_default_project(project_path: &Path) {
    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");

    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");
    assert_eq!(
        manifest_content, DEFAULT_MANIFEST_CONTENT,
        "Manifest file should be equivalent to default version 1 manifest."
    );

    let template_dir = project_path.join("template");
    assert!(template_dir.exists(), "Template directory should exist");
    assert!(
        template_dir.join("template.tex").exists(),
        "Example template file should exist"
    );

    let markdown_dir = project_path.join("Markdown");
    assert!(markdown_dir.exists(), "Markdown directory should exist");
    assert!(
        markdown_dir.join("Chapter 1 - Introduction.md").exists(),
        "Example Markdown file should exist"
    );
}

#[rstest]
fn test_init_project() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .assert()
        .success();

    assert_default_project(&project_path);
}

#[rstest]
#[case("my_project")]
#[case("./my_project")]
#[case("my_project/")]
#[case("./my_project/")]
fn test_init_project_with_custom_name(#[case] project_name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join(project_name);
    fs::create_dir(&project_path).expect("Failed to create project directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&temp_dir)
        .arg("init")
        .arg(project_name)
        .assert()
        .success();

    assert_default_project(&project_path);
}

#[rstest]
fn test_init_project_no_templates() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("-n")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");
    assert_not_contains!(manifest_content, "[[templates]]");
}

#[rstest]
fn test_init_project_with_custom_markdown_dir() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");
    let markdown_dir_name = "my_markdown_dir";

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("-m")
        .arg(markdown_dir_name)
        .assert()
        .success();

    let markdown_dir = project_path.join(markdown_dir_name);
    assert!(markdown_dir.exists(), "Markdown directory should exist");
    assert!(
        markdown_dir.join("Chapter 1 - Introduction.md").exists(),
        "Example Markdown file should exist"
    );
}

#[rstest]
fn test_init_force() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");
    fs::write(
        project_path.join("manifest.toml"),
        "Not a valid manifest file",
    )
    .expect("Failed to write manifest file");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&temp_dir)
        .arg("init")
        .arg("project")
        .arg("-f")
        .assert()
        .success();

    assert_default_project(&project_path);
}

#[rstest]
fn test_init_force_current_dir_fails() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");
    fs::write(
        project_path.join("manifest.toml"),
        "Not a valid manifest file",
    )
    .expect("Failed to write manifest file");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("-f")
        .assert()
        .failure();
}

#[rstest]
fn test_init_project_templates() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("-t")
        .arg("template.tex,booklet.tex,template_typ.typ,lix_novel_a4.tex,default_epub")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");
    assert_contains!(manifest_content, r#"name = "template.tex""#);
    assert_contains!(manifest_content, r#"name = "booklet.tex""#);
    assert_contains!(manifest_content, r#"name = "template_typ.typ""#);
    assert_contains!(manifest_content, r#"name = "lix_novel_a4.tex""#);
    assert_contains!(manifest_content, r#"name = "default_epub""#);

    let template_dir = project_path.join("template");
    assert!(template_dir.exists(), "Template directory should exist");
    assert!(
        template_dir.join("template.tex").exists(),
        "template.tex should exist in template directory."
    );
    assert!(
        template_dir.join("booklet.tex").exists(),
        "booklet.tex should exist in template directory."
    );
    assert!(
        template_dir.join("template_typ.typ").exists(),
        "template_typ.typ should exist in template directory."
    );
    assert!(
        template_dir.join("lix_novel_a4.tex").exists(),
        "lix_novel_a4.tex should exist in template directory."
    );
    assert!(
        template_dir.join("default_epub").exists(),
        "default_epub should exist in template directory."
    );
    assert!(
        template_dir.join("default_epub").is_dir(),
        "default_epub should be a directory."
    );
}

#[rstest]
fn test_init_project_smart_clean() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("--smart-clean")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");

    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(manifest_content, r#"smart_clean = true"#);
}

#[rstest]
fn test_init_project_smart_clean_threshold() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("--smart-clean-threshold")
        .arg("2")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");

    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(manifest_content, r#"smart_clean_threshold = 2"#);
}
