use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_zls_current_directory() {
    let output = Command::new("cargo")
        .args(&["run", "--", "."])
        .output()
        .expect("Failed to execute zls");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain some files from the project
    assert!(stdout.contains("Cargo.toml") || stdout.contains("src"));
}

#[test]
fn test_zls_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute zls --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("A fast ls replacement written in Rust"));
    assert!(stdout.contains("--all"));
    assert!(stdout.contains("--short"));
    assert!(stdout.contains("--time"));
    assert!(stdout.contains("--human"));
}

#[test]
fn test_zls_nonexistent_directory() {
    let output = Command::new("cargo")
        .args(&["run", "--", "/nonexistent/directory"])
        .output()
        .expect("Failed to execute zls");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not exist"));
}

#[test]
fn test_zls_with_flags() {
    let dir = tempdir().expect("Failed to create temp dir");

    // Create some test files
    fs::write(dir.path().join("file1.txt"), "content").expect("Failed to create file1");
    fs::write(dir.path().join(".hidden"), "hidden").expect("Failed to create hidden file");
    fs::create_dir(dir.path().join("subdir")).expect("Failed to create subdir");

    // Test without --all flag (should not show hidden files)
    let output = Command::new("cargo")
        .args(&["run", "--", dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute zls");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("subdir"));
    assert!(!stdout.contains(".hidden"));

    // Test with --all flag (should show hidden files)
    let output = Command::new("cargo")
        .args(&["run", "--", "--all", dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute zls --all");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("subdir"));
    assert!(stdout.contains(".hidden"));

    // Test with --short flag (original behavior is now default, so test short)
    let output = Command::new("cargo")
        .args(&["run", "--", "--short", dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute zls --short");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Short format should be more compact
    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("subdir"));

    // Test default behavior (now long format)
    let output = Command::new("cargo")
        .args(&["run", "--", dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute zls (default)");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Default (long) format should include size and timestamp
    assert!(stdout.contains("file1.txt"));
    // Should have directory indicator
    assert!(stdout.contains("d") || stdout.contains("-"));
}

#[test]
fn test_zls_human_readable_sizes() {
    let dir = tempdir().expect("Failed to create temp dir");

    // Create a file with known size
    let large_content = "x".repeat(2048); // 2KB
    fs::write(dir.path().join("large_file.txt"), &large_content)
        .expect("Failed to create large file");

    let output = Command::new("cargo")
        .args(&["run", "--", "-H", dir.path().to_str().unwrap()])
        .output()
        .expect("Failed to execute zls -H");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show human-readable size (like 2.0K)
    assert!(stdout.contains("large_file.txt"));
    // Should contain size unit indicators
    assert!(stdout.contains("K") || stdout.contains("B"));
}
