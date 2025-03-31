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

fn create_empty_project(temp_dir: &Path, threshold: Option<u32>) -> PathBuf {
    let project_path = temp_dir.join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path).arg("init").arg("-n");

    if let Some(threshold) = threshold {
        cmd.arg("--smart-clean-threshold")
            .arg(threshold.to_string());
    }

    cmd.assert().success();

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
    fs::write(
        template_dir.join(format!("{}.tex", template_name)),
        "\\documentclass{article}\n\n\\begin{document}\n\nHello, World!\n\n\\end{document}",
    )
    .expect("Failed to write example template file");
}

fn convert(project_path: &Path) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn get_conversion_folders(project_path: &Path) -> Vec<PathBuf> {
    let regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}$")
        .expect("Failed to compile regex");
    let conversion_folders = fs::read_dir(project_path)
        .expect("Failed to read project directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir()
                && regex.is_match(
                    path.file_name()
                        .expect("Failed to get file name")
                        .to_str()
                        .expect("Failed to convert file name to string"),
                )
            {
                Some(path)
            } else {
                None
            }
        });

    conversion_folders.collect()
}

#[rstest]
fn test_smart_clean() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), Some(2));

    add_template(&project_path, "tmp");

    for _ in 0..4 {
        convert(&project_path);
    }

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 4);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("smart-clean")
        .assert()
        .success();

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 2);
}

#[rstest]
fn test_smart_clean_leaves_folders() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), Some(1000));

    add_template(&project_path, "tmp");

    for _ in 0..4 {
        convert(&project_path);
    }

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 4);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("smart-clean")
        .assert()
        .success();

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 4);
}

#[rstest]
fn test_smart_clean_no_folders() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), Some(2));

    add_template(&project_path, "tmp");

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 0);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("smart-clean")
        .assert()
        .success();

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 0);
}

#[rstest]
fn test_smart_clean_no_threshold() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), None);

    add_template(&project_path, "tmp");

    for _ in 0..10 {
        convert(&project_path);
    }

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 10);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("smart-clean")
        .assert()
        .success();

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 5);
}
