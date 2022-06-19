use std::process;
use std::fs;
use std::path::PathBuf;
use std::error;
use std::str;
use assert_cmd::prelude::CommandCargoExt;
use std::env;

#[test]
fn respect_git_ignore() -> Result<(), Box<dyn error::Error>> {
    let mut tre = process::Command::cargo_bin("tre")?;
    let fixture_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "fixtures",
    ].iter().collect();
    // this path is ignored by fixtures/.gitignore
    let ignored_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "fixtures",
        "ignore_me"
    ].iter().collect();
    fs::write(ignored_path, "")?;
    env::set_current_dir(fixture_path)?;
    let output = tre.output()?.stdout;
    let text = str::from_utf8(&output)?;
    assert!(text.contains("."));
    assert!(text.contains("── .gitignore"));
    assert!(text.contains("── a"));
    assert!(text.contains("── b"));
    assert!(text.contains("── c"));
    assert!(text.contains("── d"));
    assert!(text.contains("── e"));
    assert!(text.contains("── h"));
    assert!(text.contains("── f"));
    assert!(text.contains("── g"));
    assert!(!text.contains("ignore_me"));
    Ok(())
}

#[cfg(not(windows))]
#[test]
fn ignore_hidden() -> Result<(), Box<dyn error::Error>> {
    let mut tre = process::Command::cargo_bin("tre")?;
    let fixture_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "fixtures",
    ].iter().collect();
    // this path is ignored by fixtures/.gitignore, but we aren't using .gitignore
    let ignored_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "fixtures",
        "ignore_me"
    ].iter().collect();
    fs::write(ignored_path, "")?;
    env::set_current_dir(fixture_path)?;
    let output = tre.arg("-s").output()?.stdout;
    let text = str::from_utf8(&output)?;
    assert!(text.contains("."));
    assert!(!text.contains("── .gitignore")); // hidden files should be hidden
    assert!(text.contains("── a"));
    assert!(text.contains("── b"));
    assert!(text.contains("── c"));
    assert!(text.contains("── d"));
    assert!(text.contains("── e"));
    assert!(text.contains("── h"));
    assert!(text.contains("── f"));
    assert!(text.contains("── g"));
    assert!(text.contains("ignore_me"));
    Ok(())
}

#[test]
fn all_files() -> Result<(), Box<dyn error::Error>> {
    let mut tre = process::Command::cargo_bin("tre")?;
    let fixture_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "fixtures",
    ].iter().collect();
    // this path is ignored by fixtures/.gitignore, but we aren't using .gitignore
    let ignored_path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "fixtures",
        "ignore_me"
    ].iter().collect();
    fs::write(ignored_path, "")?;
    env::set_current_dir(fixture_path)?;
    let output = tre.arg("-a").output()?.stdout;
    let text = str::from_utf8(&output)?;
    assert!(text.contains("."));
    assert!(text.contains("── .gitignore")); // hidden files should be hidden
    assert!(text.contains("── a"));
    assert!(text.contains("── b"));
    assert!(text.contains("── c"));
    assert!(text.contains("── d"));
    assert!(text.contains("── e"));
    assert!(text.contains("── h"));
    assert!(text.contains("── f"));
    assert!(text.contains("── g"));
    assert!(text.contains("ignore_me"));
    Ok(())
}
