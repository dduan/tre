use crate::file_tree::FileType;
use std::fs::Metadata;
use walkdir::{DirEntry, WalkDir};

fn find_all_paths(root: &String) -> Vec<(String, Metadata)> {
    let mut result: Vec<(String, Metadata)> = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if let Ok(meta) = entry.metadata() {
            result.push((entry.path().to_str().unwrap().to_string(), meta))
        }
    }
    result
}
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn find_non_hidden_paths(root: &String) -> Vec<(String, Metadata)> {
    let walker = WalkDir::new(root).into_iter();
    let mut result: Vec<(String, Metadata)> = Vec::new();

    for entry in walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
    {
        if let Ok(meta) = entry.metadata() {
            result.push((entry.path().to_str().unwrap().to_string(), meta))
        }
    }

    result
}
