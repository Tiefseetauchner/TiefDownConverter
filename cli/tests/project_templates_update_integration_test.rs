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

fn add_template(
    project_path: &Path,
    template_name: &str,
    template_file: &str,
    output_file: &str,
    filters: &str,
) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg(template_name)
        .arg("add")
        .arg("--template-type")
        .arg("tex")
        .arg("--template-file")
        .arg(template_file)
        .arg("--output")
        .arg(output_file)
        .arg("--filters")
        .arg(filters)
        .assert()
        .success();

    let template_dir = project_path.join("template");
    assert!(template_dir.exists(), "Template directory should exist");
    fs::write(template_dir.join(format!("{}.tex", template_name)), "")
        .expect("Failed to write example template file");
}

#[rstest]
fn test_update_template_template_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--template-file")
        .arg("new_file.tex")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "old_name"
template_type = "Tex"
template_file = "new_file.tex"
output = "old_output.tex"
filters = ["old_filters/"]"#
    );
}

#[rstest]
fn test_update_template_template_type() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--template-type")
        .arg("typst")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "old_name"
template_type = "Typst"
template_file = "old_file.tex"
output = "old_output.tex"
filters = ["old_filters/"]"#
    );
}

#[rstest]
fn test_update_template_output_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--output")
        .arg("new_output.tex")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "old_name"
template_type = "Tex"
template_file = "old_file.tex"
output = "new_output.tex"
filters = ["old_filters/"]"#
    );
}

#[rstest]
fn test_update_template_filters() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--filters")
        .arg("new_filters/")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "old_name"
template_type = "Tex"
template_file = "old_file.tex"
output = "old_output.tex"
filters = ["new_filters/"]"#
    );
}

#[rstest]
#[case(vec!["new_filter1", "new_filter2"])]
#[case(vec!["new_filter1"])]
fn test_update_template_add_filters(#[case] filters: Vec<&str>) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--add-filters")
        .arg(filters.join(","))
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        format!(
            r#"[[templates]]
name = "old_name"
template_type = "Tex"
template_file = "old_file.tex"
output = "old_output.tex"
filters = ["old_filters/"{}]"#,
            filters
                .iter()
                .map(|f| format!(r#", "{}""#, f))
                .collect::<Vec<String>>()
                .join("")
        )
        .as_str()
    );
}

#[rstest]
#[case(vec![])]
#[case(vec!["new_filter1", ""])]
#[case(vec!["new_filter1", "", "new_filter2"])]
fn test_update_template_add_filters_empty_filters(#[case] filters: Vec<&str>) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--add-filters")
        .arg(filters.join(","))
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Cannot add an empty filter to the template 'old_name'",
        ));
}

#[rstest]
fn test_update_template_remove_filters() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--remove-filters")
        .arg("old_filters/")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "old_name"
template_type = "Tex"
template_file = "old_file.tex"
output = "old_output.tex""#
    );

    assert_not_contains!(manifest_content, "filters = \"old_filters/\"");
}

#[rstest]
fn test_update_template_remove_filters_filters_remain() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/,second_filter.lua,third_filter.lua",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--remove-filters")
        .arg("old_filters/")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "old_name"
template_type = "Tex"
template_file = "old_file.tex"
output = "old_output.tex"
filters = ["second_filter.lua", "third_filter.lua"]"#
    );
}

#[rstest]
#[case(vec![])]
#[case(vec!["new_filter1", ""])]
#[case(vec!["new_filter1", "", "new_filter2"])]
fn test_update_template_remove_filters_empty_filters(#[case] filters: Vec<&str>) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--remove-filters")
        .arg(filters.join(","))
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Cannot remove an empty filter from the template 'old_name'",
        ));
}

#[rstest]
fn test_update_template_all_fields() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("old_name")
        .arg("update")
        .arg("--template-file")
        .arg("new_file.tex")
        .arg("--template-type")
        .arg("typst")
        .arg("--output")
        .arg("new_output.tex")
        .arg("--filters")
        .arg("new_filters/")
        .assert()
        .success();

    let manifest_path = project_path.join("manifest.toml");
    assert!(manifest_path.exists(), "Manifest file should exist");
    let manifest_content = fs::read_to_string(manifest_path).expect("Failed to read manifest file");

    assert_contains!(
        manifest_content,
        r#"[[templates]]
name = "old_name"
template_type = "Typst"
template_file = "new_file.tex"
output = "new_output.tex"
filters = ["new_filters/"]"#
    );
}

#[rstest]
fn test_update_template_non_existing_template() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());
    add_template(
        &project_path,
        "old_name",
        "old_file.tex",
        "old_output.tex",
        "old_filters/",
    );
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg("non_existing_template")
        .arg("update")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Template with name 'non_existing_template' does not exist.",
        ));
}
