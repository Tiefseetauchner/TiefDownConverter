// NOTE: Deactivated as these will only work after the manifest migration.

use crate::{
    _tests::tests_common::get_default_manifest,
    injections::{add_files_to_injection, add_injection, remove_injection},
    manifest_model::Injection,
};
use rstest::rstest;
use std::path::PathBuf;

#[rstest]
#[case("snake_case")]
#[case("Injection Name")]
#[case("Sp3cia/ $ha%acters")]
fn test_add_injection_injections_none(#[case] name: &str) {
    let mut manifest = get_default_manifest();

    add_injection(
        &mut manifest,
        name.to_string(),
        vec![PathBuf::from("file1.txt"), PathBuf::from("file2.md")],
    )
    .expect("An error occurred adding the injection to the manifest.");

    let injections = manifest.injections;
    assert!(injections.is_some(), "Injections was None");

    if let Some(injections) = injections {
        assert_eq!(injections.len(), 1);
        assert_eq!(injections[0].name, name);
        assert_eq!(injections[0].files[0].to_string_lossy(), "file1.txt");
        assert_eq!(injections[0].files[1].to_string_lossy(), "file2.md");
    }
}

#[rstest]
fn test_add_injection_injections_others_exist() {
    let mut manifest = get_default_manifest();
    manifest.injections = Some(vec![Injection {
        name: "other_injection".to_string(),
        files: Vec::new(),
    }]);

    add_injection(
        &mut manifest,
        "injection_name".to_string(),
        vec![PathBuf::from("file1.txt"), PathBuf::from("file2.md")],
    )
    .expect("An error occurred adding the injection to the manifest.");

    let injections = manifest.injections;
    assert!(injections.is_some(), "Injections was None");

    if let Some(injections) = injections {
        assert_eq!(injections.len(), 2);
        assert_eq!(injections[0].name, "injection_name");
        assert_eq!(injections[0].files[0].to_string_lossy(), "file1.txt");
        assert_eq!(injections[0].files[1].to_string_lossy(), "file2.md");
    }
}

#[rstest]
fn test_add_injection_injections_exists_already() {
    let mut manifest = get_default_manifest();
    manifest.injections = Some(vec![Injection {
        name: "injection_name".to_string(),
        files: Vec::new(),
    }]);

    let err = add_injection(
        &mut manifest,
        "injection_name".to_string(),
        vec![PathBuf::from("file1.txt"), PathBuf::from("file2.md")],
    )
    .expect_err("Adding injection did not error.");

    assert_eq!(
        err.to_string(),
        "Injection 'injection_name' already exists."
    )
}

#[rstest]
#[case("snake_case")]
#[case("Injection Name")]
#[case("Sp3cia/ $ha%acters")]
fn test_remove_injection(#[case] name: &str) {
    let mut manifest = get_default_manifest();
    manifest.injections = Some(vec![Injection {
        name: name.to_string(),
        files: Vec::new(),
    }]);

    remove_injection(&mut manifest, name.to_string())
        .expect("An error occurred removing the injection from the manifest.");

    let injections = manifest.injections;
    assert!(injections.is_some(), "Injections was None");

    if let Some(injections) = injections {
        assert_eq!(injections.len(), 0);
    }
}

#[rstest]
#[case("snake_case")]
#[case("Injection Name")]
#[case("Sp3cia/ $ha%acters")]
fn test_remove_injection_does_not_exist(#[case] name: &str) {
    let mut manifest = get_default_manifest();
    manifest.injections = Some(vec![Injection {
        name: "other_injection".to_string(),
        files: Vec::new(),
    }]);

    let err = remove_injection(&mut manifest, name.to_string())
        .expect_err("Removing injection did not fail.");

    assert_eq!(
        err.to_string(),
        format!("Injection '{}' was not found in the manifest.", name)
    )
}

#[rstest]
#[case("snake_case")]
#[case("Injection Name")]
#[case("Sp3cia/ $ha%acters")]
fn test_add_files_to_injection(#[case] name: &str) {
    let mut manifest = get_default_manifest();
    manifest.injections = Some(vec![Injection {
        name: "other_injection".to_string(),
        files: vec![PathBuf::from("file0.txt")],
    }]);

    add_files_to_injection(
        &mut manifest,
        name.to_string(),
        vec![PathBuf::from("file1.txt"), PathBuf::from("file2.md")],
    )
    .expect("An error occurred adding files to the injection in the manifest.");

    let injections = manifest.injections;
    assert!(injections.is_some(), "Injections was None");

    if let Some(injections) = injections {
        assert_eq!(injections.len(), 1);
        assert_eq!(injections[0].name, name);
        assert_eq!(injections[0].files[0].to_string_lossy(), "file0.txt");
        assert_eq!(injections[0].files[1].to_string_lossy(), "file1.txt");
        assert_eq!(injections[0].files[2].to_string_lossy(), "file2.md");
    }
}
