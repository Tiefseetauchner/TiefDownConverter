use assert_cmd::Command;
use rstest::rstest;

#[rstest]
fn test_check_dependencies_success() {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.arg("check-dependencies").assert().success();
}
