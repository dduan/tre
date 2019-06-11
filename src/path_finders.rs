use crate::file_tree::FileType;
use std::fs;
use std::process::Command;
use walkdir::{DirEntry, WalkDir};

pub fn find_all_paths(root: &String) -> Vec<(String, FileType)> {
    let mut result: Vec<(String, FileType)> = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if let Ok(meta) = entry.metadata() {
            let path = entry.path().to_str().unwrap().to_string();
            if &path != root {
                result.push((path, FileType::new(meta)))
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

pub fn find_non_hidden_paths(root: &String) -> Vec<(String, FileType)> {
    let walker = WalkDir::new(root).into_iter();
    let mut result: Vec<(String, FileType)> = Vec::new();

    for entry in walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        if let Ok(meta) = entry.metadata() {
            let path = entry.path().to_str().unwrap().to_string();
            if &path != root {
                result.push((path, FileType::new(meta)))
            }
        } else {
            println!("falied to get meta");
        }
    }
    result
}

pub fn find_non_git_ignored_paths(root: &String) -> Vec<(String, FileType)> {
    if let Ok(git_output) = Command::new("git").arg("ls-files").arg(root).output() {
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

    return find_non_hidden_paths(root);
}
