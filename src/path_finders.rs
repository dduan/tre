use crate::file_tree::FileType;
use std::process::Command;
use std::{fs, path};
use walkdir::{DirEntry, WalkDir};

pub fn find_all_paths(
    root: &str,
    directories_only: bool,
    max_depth: usize,
) -> Vec<(String, FileType)> {
    let mut result: Vec<(String, FileType)> = Vec::new();
    for entry in WalkDir::new(root)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Ok(meta) = entry.metadata() {
            if directories_only && !meta.is_dir() {
                continue;
            }

            if let Some(path) = entry.path().to_str() {
                let path = path.to_string();
                if path != root {
                    result.push((path, FileType::new(meta)))
                }
            }
        }
    }
    result
}

fn is_hidden(name: &str) -> bool {
    name != "." && name.starts_with('.') && name != ".."
}

fn should_include(entry: &DirEntry, root: &str) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !is_hidden(s) || s == root)
        .unwrap_or(true)
}

pub fn find_non_hidden_paths(
    root: &str,
    directories_only: bool,
    max_depth: usize,
) -> Vec<(String, FileType)> {
    let walker = WalkDir::new(root).max_depth(max_depth).into_iter();
    let mut result: Vec<(String, FileType)> = Vec::new();

    for entry in walker
        .filter_entry(|e| should_include(e, root))
        .filter_map(|e| e.ok())
    {
        if let Ok(meta) = entry.metadata() {
            if directories_only && !meta.is_dir() {
                continue;
            }
            if let Some(path) = entry.path().to_str() {
                let path = path.to_string();
                if path != root {
                    result.push((path, FileType::new(meta)))
                }
            }
        }
    }
    result
}

pub fn find_non_git_ignored_paths(
    root: &str,
    directories_only: bool,
    max_depth: usize,
) -> Vec<(String, FileType)> {
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
                    .split('\n')
                    .filter_map(|p| {
                        let path_string = if max_depth != std::usize::MAX {
                            path::Path::new(p)
                                .components()
                                .take(max_depth)
                                .collect::<path::PathBuf>()
                                .as_path()
                                .to_str()
                                .unwrap()
                                .to_string()
                        } else {
                            p.to_string()
                        };
                        fs::metadata(&path_string)
                            .map(|m| (path_string, FileType::new(m)))
                            .ok()
                    })
                    .collect();
            }
        }
    }

    find_non_hidden_paths(root, directories_only, max_depth)
}
