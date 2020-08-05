use crate::file_tree::FileType;
use std::fs;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

pub fn find_all_paths(root: &str, directories_only: bool) -> Vec<(String, FileType)> {
    let mut result: Vec<(String, FileType)> = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if let Ok(meta) = entry.metadata() {
            if directories_only && !meta.is_dir() {
                continue
            }

            if let Some(path) = entry.path().to_str() {
                let path = path.to_string();
                if &path != root {
                    result.push((path, FileType::new(meta)))
                }
            }
        }
    }
    result
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s != "." && s.starts_with(".") && s != "..")
        .unwrap_or(false)
}

pub fn find_non_hidden_paths(root: &str, directories_only: bool) -> Vec<(String, FileType)> {
    let walker = WalkDir::new(root).into_iter();
    let mut result: Vec<(String, FileType)> = Vec::new();

    for entry in walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        if let Ok(meta) = entry.metadata() {
            if directories_only && !meta.is_dir() {
                continue
            }
            if let Some(path) = entry.path().to_str() {
                let path = path.to_string();
                if &path != root {
                    result.push((path, FileType::new(meta)))
                }
            }
        }
    }
    result
}

pub fn find_non_git_ignored_paths(root: &str, directories_only: bool) -> Vec<(String, FileType)> {
    let mut git_command = Command::new("git");
    if directories_only {
        git_command
            .arg("ls-tree")
            .arg("-r")
            .arg("-d")
            .arg("--name-only")
            .arg("HEAD")
            .arg(root);
    } else {
        git_command.arg("ls-files").arg(root);
    };

    if let Ok(git_output) = git_command.output() {
        if git_output.status.success() {
            if let Ok(paths_buf) = String::from_utf8(git_output.stdout) {
                return paths_buf
                    .split("\n")
                    .into_iter()
                    .filter_map(|p| {
                        fs::metadata(&p)
                            .map(|m| (p.to_string(), FileType::new(m)))
                            .ok()
                    })
                    .collect();
            }
        }
    }

    return find_non_hidden_paths(root, directories_only);
}
