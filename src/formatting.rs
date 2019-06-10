use super::file_tree::{File, FileTree, FileType};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum PrefixSegment {
    ShapeL, // "└── "
    ShapeT, // "├── "
    ShapeI, // "│   "
    Empty,  // "    "
}

#[derive(Debug, Clone, PartialEq)]
pub struct FormattedEntry {
    pub file_type: FileType,
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
        let count = format_history.get(&ancestor.id).unwrap_or(&0);
        if *count == ancestor.children_count() {
            segments.push(PrefixSegment::Empty);
        } else {
            segments.push(PrefixSegment::ShapeI);
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
) {
    let prefix = make_prefix(tree, file, &format_history);
    result.push(FormattedEntry {
        file_type: file.file_type.clone(),
        name: file.display_name.clone(),
        path: file.path.clone(),
        prefix: prefix,
        link: file.link(),
    });

    if let Some(parent) = tree.get_parent(file) {
        if let Some(&n) = format_history.get(&parent.id) {
            format_history.insert(parent.id, n + 1);
        }
    }

    if let FileType::Directory = file.file_type {
        format_history.insert(file.id, 0);
    }

    if let Some(children) = file.children() {
        for child_id in children.values() {
            format_file(tree, tree.get(*child_id), format_history, result);
        }
    }
}

pub fn format_paths(root_path: String, children: Vec<(String, FileType)>) -> Vec<FormattedEntry> {
    let tree = FileTree::new(root_path, children);
    let mut history = HashMap::new();
    let mut result = Vec::new();
    let root = tree.get_root();
    format_file(&tree, root, &mut history, &mut result);
    result
}

#[cfg(test)]
mod test {
    use super::FormattedEntry;
    use crate::file_tree::FileType;

    #[test]
    fn formatting_works() {
        let formatted = super::format_paths(
            String::from("."),
            vec![
                (String::from("a"), FileType::File),
                (String::from("b/c"), FileType::File),
            ],
        );

        let variant0 = vec![
            FormattedEntry {
                file_type: FileType::Directory,
                name: String::from("."),
                path: String::from("."),
                prefix: String::new(),
                link: None,
            },
            FormattedEntry {
                file_type: FileType::File,
                name: String::from("a"),
                path: String::from("a"),
                prefix: String::from("├── "),
                link: None,
            },
            FormattedEntry {
                file_type: FileType::Directory,
                name: String::from("b"),
                path: String::from("./b"),
                prefix: String::from("└── "),
                link: None,
            },
            FormattedEntry {
                file_type: FileType::File,
                name: String::from("c"),
                path: String::from("b/c"),
                prefix: String::from("    └── "),
                link: None,
            },
        ];

        let variant1 = vec![
            FormattedEntry {
                file_type: FileType::Directory,
                name: String::from("."),
                path: String::from("."),
                prefix: String::new(),
                link: None,
            },
            FormattedEntry {
                file_type: FileType::Directory,
                name: String::from("b"),
                path: String::from("./b"),
                prefix: String::from("├── "),
                link: None,
            },
            FormattedEntry {
                file_type: FileType::File,
                name: String::from("c"),
                path: String::from("b/c"),
                prefix: String::from("│   └── "),
                link: None,
            },
            FormattedEntry {
                file_type: FileType::File,
                name: String::from("a"),
                path: String::from("a"),
                prefix: String::from("└── "),
                link: None,
            },
        ];

        assert!(formatted == variant0 || formatted == variant1);
    }
}
