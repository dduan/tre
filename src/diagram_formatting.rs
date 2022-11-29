use super::file_tree::{File, FileTree, FileType};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
enum PrefixSegment {
    ShapeL, // "└── "
    ShapeT, // "├── "
    ShapeI, // "│   "
    Empty,  // "    "
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormattedEntry {
    pub name: String,
    pub path: String,
    pub prefix: String,
    pub link: Option<String>,
}

fn make_prefix(tree: &FileTree, file: &File, format_history: &HashMap<usize, usize>) -> String {
    let mut segments = Vec::new();
    let mut current = file;
    if let Some(ancestor) = tree.get_parent(file) {
        let count = format_history.get(&ancestor.id).unwrap_or(&0);
        if *count >= ancestor.children_count() - 1 {
            segments.push(PrefixSegment::ShapeL);
        } else {
            segments.push(PrefixSegment::ShapeT);
        }
        current = ancestor;
    }

    while let Some(ancestor) = tree.get_parent(current) {
        match format_history.get(&ancestor.id) {
            Some(count) if *count == ancestor.children_count() => {
                segments.push(PrefixSegment::Empty)
            }
            Some(_) => segments.push(PrefixSegment::ShapeI),
            None => {}
        }
        current = ancestor;
    }

    segments.reverse();
    segments.iter().fold(String::new(), |s, seg| {
        s + match seg {
            PrefixSegment::ShapeL => "└── ",
            PrefixSegment::ShapeT => "├── ",
            PrefixSegment::ShapeI => "│   ",
            PrefixSegment::Empty => "    ",
        }
    })
}

fn format_file(
    tree: &FileTree,
    file: &File,
    format_history: &mut HashMap<usize, usize>,
    result: &mut Vec<FormattedEntry>,
    make_absolute: bool,
    collapse: bool,
) {
    let mut current = file;

    let prefix = make_prefix(tree, current, format_history);
    let mut path = if make_absolute {
        fs::canonicalize(&current.path)
            .unwrap()
            .display()
            .to_string()
    } else {
        current.path.clone()
    };
    let mut name = current.display_name.clone();

    if collapse {
        while current.children_count() == 1 {
            let child = current.children().unwrap().values().next().unwrap();
            let child_node = tree.get(*child);
            let child_name = child_node.display_name.clone();
            name = Path::new(&name).join(child_name).display().to_string();
            path = child_node.path.clone();
            current = child_node;
        }
    }

    result.push(FormattedEntry {
        name,
        path,
        prefix,
        link: current.link(),
    });

    if let Some(parent) = tree.get_parent(file) {
        if let Some(&n) = format_history.get(&parent.id) {
            format_history.insert(parent.id, n + 1);
        }
    }

    if let FileType::Directory = current.file_type {
        format_history.insert(current.id, 0);
    }

    if let Some(children) = current.children() {
        for child_id in children.values() {
            format_file(
                tree,
                tree.get(*child_id),
                format_history,
                result,
                make_absolute,
                collapse,
            );
        }
    }
}

pub fn format_paths(
    root_path: &str,
    children: Vec<(String, FileType)>,
    make_absolute: bool,
    collapse: bool,
) -> Vec<FormattedEntry> {
    let mut history = HashMap::new();
    let mut result = Vec::new();
    match FileTree::new(root_path, children) {
        Some(tree) => {
            let root = tree.get_root();
            format_file(
                &tree,
                root,
                &mut history,
                &mut result,
                make_absolute,
                collapse,
            );
            result
        }
        None => Vec::new(),
    }
}

#[cfg(test)]
mod test {
    use super::FormattedEntry;
    use crate::file_tree::FileType;
    use std::path;

    #[test]
    fn formatting_works() {
        let formatted = super::format_paths(
            ".",
            vec![
                ("a".to_string(), FileType::File),
                (format!("b{}c", path::MAIN_SEPARATOR), FileType::File),
            ],
            false,
            false,
        );

        let bc_path = format!("b{}c", path::MAIN_SEPARATOR);
        let b_path = format!(".{}b", path::MAIN_SEPARATOR);
        let variant0 = vec![
            FormattedEntry {
                name: ".".to_string(),
                path: ".".to_string(),
                prefix: String::new(),
                link: None,
            },
            FormattedEntry {
                name: "a".to_string(),
                path: "a".to_string(),
                prefix: "├── ".to_string(),
                link: None,
            },
            FormattedEntry {
                name: "b".to_string(),
                path: b_path.clone(),
                prefix: "└── ".to_string(),
                link: None,
            },
            FormattedEntry {
                name: "c".to_string(),
                path: bc_path.clone(),
                prefix: "    └── ".to_string(),
                link: None,
            },
        ];

        let variant1 = vec![
            FormattedEntry {
                name: ".".to_string(),
                path: ".".to_string(),
                prefix: String::new(),
                link: None,
            },
            FormattedEntry {
                name: "b".to_string(),
                path: b_path.clone(),
                prefix: "├── ".to_string(),
                link: None,
            },
            FormattedEntry {
                name: "c".to_string(),
                path: bc_path.clone(),
                prefix: "│   └── ".to_string(),
                link: None,
            },
            FormattedEntry {
                name: "a".to_string(),
                path: "a".to_string(),
                prefix: "└── ".to_string(),
                link: None,
            },
        ];

        assert!(formatted == variant0 || formatted == variant1);
    }
}
