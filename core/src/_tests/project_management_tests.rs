use rstest::rstest;

use crate::project_management::check_dependencies;

#[rstest]
fn check_dependencies_valid() {
    let dependencies = vec!["ls", "echo"];
    assert!(check_dependencies(dependencies).is_ok());
}

#[rstest]
fn check_dependencies_invalid() {
    let dependencies = vec!["ls", "invalid_command_that_no_sane_person_would_have"];
    assert!(check_dependencies(dependencies).is_err());
}
