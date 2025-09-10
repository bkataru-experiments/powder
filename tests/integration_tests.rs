use assert_cmd::Command;
use predicates::prelude::*;
use std::env;
use tempfile::tempdir;

#[test]
fn test_basic_pwd_functionality() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("powder"));
}

#[test]
fn test_help_output() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("modern, colorful alternative to pwd"))
        .stdout(predicate::str::contains("--themes"))
        .stdout(predicate::str::contains("--history"))
        .stdout(predicate::str::contains("--theme"));
}

#[test]
fn test_version_output() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_themes_listing() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--themes")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available Themes"))
        .stdout(predicate::str::contains("default"))
        .stdout(predicate::str::contains("ocean"))
        .stdout(predicate::str::contains("forest"))
        .stdout(predicate::str::contains("sunset"))
        .stdout(predicate::str::contains("mono"));
}

#[test]
fn test_valid_theme_selection() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--theme").arg("ocean")
        .assert()
        .success();
}

#[test]
fn test_invalid_theme_selection() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--theme").arg("nonexistent")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Theme 'nonexistent' not found"));
}

#[test]
fn test_history_output() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--history")
        .assert()
        .success()
        .stdout(predicate::str::contains("Directory History").or(predicate::str::contains("No directory history")));
}

#[test]
fn test_verbose_output() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("Directory Information"))
        .stdout(predicate::str::contains("Path:"));
}

#[test]
fn test_git_info() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--git")
        .assert()
        .success()
        .stdout(predicate::str::contains("Git Repository Information").or(predicate::str::is_empty()));
}

#[test]
fn test_no_color_option() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--no-color")
        .assert()
        .success();
}

#[test]
fn test_custom_separator() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--separator").arg(" → ")
        .assert()
        .success()
        .stdout(predicate::str::contains(" → "));
}

#[test]
fn test_logical_and_physical_conflict() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--logical").arg("--physical")
        .assert()
        .failure()
        .stderr(predicate::str::contains("mutually exclusive"));
}

#[test]
fn test_logical_option() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--logical")
        .assert()
        .success();
}

#[test]
fn test_physical_option() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--physical")
        .assert()
        .success();
}

#[test]
fn test_combined_options() {
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.arg("--verbose")
        .arg("--git")
        .arg("--theme").arg("forest")
        .assert()
        .success()
        .stdout(predicate::str::contains("Directory Information"));
}

#[test]
fn test_pd_alias() {
    let mut cmd = Command::cargo_bin("pd").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("powder"));
}

#[test]
fn test_responsiveness_with_long_path() {
    // Create a temporary directory with a long path
    let temp_dir = tempdir().unwrap();
    let deep_path = temp_dir.path()
        .join("very")
        .join("deep")
        .join("directory")
        .join("structure")
        .join("that")
        .join("should")
        .join("be")
        .join("handled")
        .join("gracefully");
    
    std::fs::create_dir_all(&deep_path).unwrap();
    
    let mut cmd = Command::cargo_bin("powder").unwrap();
    cmd.current_dir(&deep_path)
        .assert()
        .success();
}

#[test]
fn test_history_persistence() {
    // Test that history is maintained across invocations
    let temp_dir = tempdir().unwrap();
    
    // First invocation
    let mut cmd1 = Command::cargo_bin("powder").unwrap();
    cmd1.current_dir(&temp_dir)
        .assert()
        .success();
    
    // Second invocation to check history
    let mut cmd2 = Command::cargo_bin("powder").unwrap();
    cmd2.current_dir(&temp_dir)
        .arg("--history")
        .assert()
        .success()
        .stdout(predicate::str::contains("Directory History"));
}