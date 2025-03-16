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

    project_path
}

#[rstest]
fn test_add_template() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg("my_template.tex")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "my_template.tex"
template_type = "Tex""#
    );
}

#[rstest]
#[case("Tex")]
#[case("Typst")]
#[case("Epub")]
fn test_add_template_with_custom_type(#[case] template_type: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg("My custom template")
        .arg("-f")
        .arg("my_template.template")
        .arg("-t")
        .arg(template_type.to_lowercase())
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        format!(
            r#"[[templates]]
name = "My custom template"
template_type = "{}"
template_file = "my_template.template""#,
            template_type
        )
        .as_str()
    );
}

#[rstest]
#[case("output.pdf")]
#[case("output.html")]
#[case("folder/output.pdf")]
fn test_add_template_with_custom_output(#[case] output_name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg("test.tex")
        .arg("-o")
        .arg(output_name)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        format!(
            r#"[[templates]]
name = "test.tex"
template_type = "Tex"
output = "{}""#,
            output_name
        )
        .as_str()
    );
}

#[rstest]
#[case("test.tex")]
#[case("test.typ")]
#[case("test_epub")]
#[case("folder/test.tex")]
#[case("folder/test.typ")]
#[case("folder/test_epub")]
fn test_add_template_with_custom_file_path(#[case] template_path: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg("test.tex")
        .arg("-f")
        .arg(template_path)
        .arg("-t")
        .arg("tex")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        format!(
            r#"[[templates]]
name = "test.tex"
template_type = "Tex"
template_file = "{}""#,
            template_path
        )
        .as_str()
    );
}

#[rstest]
#[case("test.tex", "Tex")]
#[case("test.typ", "Typst")]
#[case("test_epub", "Epub")]
#[case("folder/test.tex", "Tex")]
#[case("folder/test.typ", "Typst")]
#[case("folder/test_epub", "Epub")]
fn test_add_template_with_name_chooses_correct_template_type(
    #[case] template_name: &str,
    #[case] template_type: &str,
) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg(template_name)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        format!(
            r#"[[templates]]
name = "{}"
template_type = "{}""#,
            template_name, template_type
        )
        .as_str()
    );
}

#[rstest]
#[case("test.tex", "Tex")]
#[case("test.typ", "Typst")]
#[case("test_epub", "Epub")]
#[case("folder/test.tex", "Tex")]
#[case("folder/test.typ", "Typst")]
#[case("folder/test_epub", "Epub")]
fn test_add_template_with_file_chooses_correct_template_type(
    #[case] template_file: &str,
    #[case] template_type: &str,
) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg("Custom Template Name")
        .arg("-f")
        .arg(template_file)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        format!(
            r#"[[templates]]
name = "Custom Template Name"
template_type = "{}"
template_file = "{}""#,
            template_type, template_file
        )
        .as_str()
    );
}

#[rstest]
fn test_add_template_with_filters() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("add-template")
        .arg("test.tex")
        .arg("--filters")
        .arg("testfilter.lua,superfilters/,my_filters/the_filter.lua")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "test.tex"
template_type = "Tex"
filters = ["testfilter.lua", "superfilters/", "my_filters/the_filter.lua"]"#
    );
}
