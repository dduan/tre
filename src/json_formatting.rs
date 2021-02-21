use super::file_tree::{File, FileTree, FileType, TypeSpecficData};
use serde::Serialize;

#[derive(Serialize)]
struct SerializableTreeFile<'a> {
    pub name: &'a String,
    pub path: &'a String,
}

#[derive(Serialize)]
struct SerializableTreeLink<'a> {
    pub name: &'a String,
    pub path: &'a String,
    pub link: String,
}

#[derive(Serialize)]
struct SerializableTreeDirectory<'a> {
    pub name: &'a String,
    pub path: &'a String,
    pub contents: Vec<SerializableTreeNode<'a>>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum SerializableTreeNode<'a> {
    File(SerializableTreeFile<'a>),
    Link(SerializableTreeLink<'a>),
    Directory(Box<SerializableTreeDirectory<'a>>),
}

impl SerializableTreeNode<'_> {
    pub fn new(tree: &FileTree) -> SerializableTreeNode {
        SerializableTreeNode::from(tree, &tree.storage[tree.root_id])
    }

    fn from<'a>(tree: &'a FileTree, file: &'a File) -> SerializableTreeNode<'a> {
        match &file.data {
            TypeSpecficData::File => SerializableTreeNode::File(SerializableTreeFile {
                name: &file.display_name,
                path: &file.path,
            }),
            TypeSpecficData::Directory(map) => {
                SerializableTreeNode::Directory(Box::new(SerializableTreeDirectory {
                    name: &file.display_name,
                    path: &file.path,
                    contents: map
                        .values()
                        .map(|id| SerializableTreeNode::from(tree, &tree.storage[*id]))
                        .collect(),
                }))
            }
            TypeSpecficData::Link(link) => SerializableTreeNode::Link(SerializableTreeLink {
                name: &file.display_name,
                path: &file.path,
                link: link.clone(),
            }),
        }
    }
}

pub fn format_paths(root_path: &str, children: Vec<(String, FileType)>) -> String {
    match FileTree::new(root_path, children) {
        Some(tree) => {
            let node = SerializableTreeNode::new(&tree);
            serde_json::to_string_pretty(&node).unwrap_or_else(|_| "{}".to_string())
        }
        None => "{}".to_string(),
    }
}
