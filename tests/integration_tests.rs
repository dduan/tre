use std::process;
use std::fs;
use std::path::PathBuf;
use std::error;
use std::str;
use assert_cmd::prelude::CommandCargoExt;
use std::env;

// on Windows, piped output uses ascii for formatting instead of unicode.
// this function makes the tests check for ascii formatting on Windows and
// unicode formatting on other platforms
fn has_entry(text: &str, name: &str) -> bool {
    if cfg!(windows) {
        text.contains(&format!("-- {}", name))
    } else {
        text.contains(&format!("── {}", name))
    }
}

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
    assert!(has_entry(text, ".gitignore"));
    assert!(has_entry(text, "a"));
    assert!(has_entry(text, "b"));
    assert!(has_entry(text, "c"));
    assert!(has_entry(text, "d"));
    assert!(has_entry(text, "e"));
    assert!(has_entry(text, "h"));
    assert!(has_entry(text, "f"));
    assert!(has_entry(text, "g"));
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
    assert!(!has_entry(text, ".gitignore")); // hidden files should be hidden
    assert!(has_entry(text, "a"));
    assert!(has_entry(text, "b"));
    assert!(has_entry(text, "c"));
    assert!(has_entry(text, "d"));
    assert!(has_entry(text, "e"));
    assert!(has_entry(text, "h"));
    assert!(has_entry(text, "f"));
    assert!(has_entry(text, "g"));
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
    assert!(has_entry(text, ".gitignore"));
    assert!(has_entry(text, "a"));
    assert!(has_entry(text, "b"));
    assert!(has_entry(text, "c"));
    assert!(has_entry(text, "d"));
    assert!(has_entry(text, "e"));
    assert!(has_entry(text, "h"));
    assert!(has_entry(text, "f"));
    assert!(has_entry(text, "g"));
    assert!(text.contains("ignore_me"));
    Ok(())
}
