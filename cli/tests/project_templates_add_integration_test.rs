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
        .arg("templates")
        .arg("my_template.tex")
        .arg("add")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    assert!(
        manifest
            .templates
            .iter()
            .any(|t| t.name == "my_template.tex"
                && t.template_type == tiefdownlib::template_type::TemplateType::Tex)
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
        .arg("templates")
        .arg("My custom template")
        .arg("add")
        .arg("-f")
        .arg("my_template.template")
        .arg("-t")
        .arg(template_type.to_lowercase())
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let tmpl = manifest
        .templates
        .iter()
        .find(|t| t.name == "My custom template")
        .unwrap();
    assert_eq!(
        tmpl.template_type,
        tiefdownlib::template_type::TemplateType::from(template_type)
    );
    assert_eq!(
        tmpl.template_file.as_ref().unwrap().to_str(),
        Some("my_template.template")
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
        .arg("templates")
        .arg("test.tex")
        .arg("add")
        .arg("-o")
        .arg(output_name)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let tmpl = manifest
        .templates
        .iter()
        .find(|t| t.name == "test.tex")
        .unwrap();
    assert_eq!(
        tmpl.template_type,
        tiefdownlib::template_type::TemplateType::Tex
    );
    assert_eq!(tmpl.output.as_ref().unwrap().to_str(), Some(output_name));
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
        .arg("templates")
        .arg("test.tex")
        .arg("add")
        .arg("-f")
        .arg(template_path)
        .arg("-t")
        .arg("tex")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let tmpl = manifest
        .templates
        .iter()
        .find(|t| t.name == "test.tex")
        .unwrap();
    assert_eq!(
        tmpl.template_type,
        tiefdownlib::template_type::TemplateType::Tex
    );
    assert_eq!(
        tmpl.template_file.as_ref().unwrap().to_str(),
        Some(template_path)
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
        .arg("templates")
        .arg(template_name)
        .arg("add")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let tmpl = manifest
        .templates
        .iter()
        .find(|t| t.name == template_name)
        .unwrap();
    assert_eq!(
        tmpl.template_type,
        tiefdownlib::template_type::TemplateType::from(template_type)
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
        .arg("templates")
        .arg("Custom Template Name")
        .arg("add")
        .arg("-f")
        .arg(template_file)
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let tmpl = manifest
        .templates
        .iter()
        .find(|t| t.name == "Custom Template Name")
        .unwrap();
    assert_eq!(
        tmpl.template_type,
        tiefdownlib::template_type::TemplateType::from(template_type)
    );
    assert_eq!(
        tmpl.template_file.as_ref().unwrap().to_str(),
        Some(template_file)
    );
}

#[rstest]
fn test_add_template_with_filters() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("test.tex")
        .arg("add")
        .arg("--filters")
        .arg("testfilter.lua,superfilters/,my_filters/the_filter.lua")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest = assertions::read_manifest(&manifest_path);
    let tmpl = manifest
        .templates
        .iter()
        .find(|t| t.name == "test.tex")
        .unwrap();
    assert_eq!(
        tmpl.template_type,
        tiefdownlib::template_type::TemplateType::Tex
    );
    assert_eq!(
        tmpl.filters.as_ref().unwrap(),
        &vec![
            "testfilter.lua",
            "superfilters/",
            "my_filters/the_filter.lua"
        ]
    );
}
