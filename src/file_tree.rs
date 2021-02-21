use slab::Slab;
use std::collections::HashMap;
use std::fs::{self, Metadata};
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Link,
}

impl FileType {
    pub fn new(meta: Metadata) -> FileType {
        let t = meta.file_type();
        if t.is_dir() {
            FileType::Directory
        } else if t.is_symlink() {
            FileType::Link
        } else {
            FileType::File
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeSpecficData {
    File,
    Directory(HashMap<String, usize>),
    Link(String),
}

#[derive(Debug, Clone)]
pub struct File {
    pub id: usize,
    parent: Option<usize>,
    pub display_name: String,
    pub path: String,
    pub file_type: FileType,
    pub data: TypeSpecficData,
}

impl File {
    pub fn children_count(&self) -> usize {
        if let TypeSpecficData::Directory(children) = &self.data {
            children.len()
        } else {
            0
        }
    }

    pub fn children(&self) -> Option<&HashMap<String, usize>> {
        if let TypeSpecficData::Directory(children) = &self.data {
            Some(children)
        } else {
            None
        }
    }

    pub fn link(&self) -> Option<String> {
        if let TypeSpecficData::Link(link) = &self.data {
            Some(link.clone())
        } else {
            None
        }
    }

    fn child_key(&self, name: &str) -> Option<usize> {
        if let TypeSpecficData::Directory(children) = &self.data {
            children.get(name).cloned()
        } else {
            None
        }
    }

    fn add_child(&mut self, name: &str, id: usize) {
        if let TypeSpecficData::Directory(children) = &mut self.data {
            children.insert(name.to_string(), id);
        }
    }

    #[cfg(test)]
    fn is_file(&self) -> bool {
        if let TypeSpecficData::File = self.data {
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    fn is_dir(&self) -> bool {
        if let TypeSpecficData::Directory(_) = self.data {
            true
        } else {
            false
        }
    }
}

pub struct FileTree {
    pub storage: Slab<Box<File>>,
    pub root_id: usize,
}

impl FileTree {
    pub fn new(root_path: &str, children: Vec<(String, FileType)>) -> Option<FileTree> {
        let mut slab = Slab::new();
        let root_entry = slab.vacant_entry();
        let root_id = root_entry.key();

        let root_prefix_len: usize = Path::new(root_path)
            .components()
            .filter(|c| match c {
                Component::CurDir => false,
                _ => true,
            })
            .collect::<Vec<_>>()
            .len();

        let root = Box::new(File {
            id: root_id,
            parent: None,
            display_name: root_path.to_string(),
            path: root_path.to_string(),
            file_type: FileType::Directory,
            data: TypeSpecficData::Directory(HashMap::new()),
        });
        root_entry.insert(root);

        for (path, meta) in children {
            let data_option: Option<TypeSpecficData> = match meta {
                FileType::Link => fs::read_link(&path)
                    .ok()
                    .and_then(|path| path.to_str().map(|x| x.to_string()))
                    .map(|path| TypeSpecficData::Link(path)),
                FileType::Directory => Some(TypeSpecficData::Directory(HashMap::new())),
                FileType::File => Some(TypeSpecficData::File),
            };

            if data_option.is_none() {
                return None;
            }

            let data = data_option.unwrap();

            let mut ancestry: Vec<Component> = Path::new(&path)
                .components()
                .filter(|c| {
                    if let Component::CurDir = c {
                        false
                    } else {
                        true
                    }
                })
                .skip(root_prefix_len)
                .collect();

            let ancestor = ancestry
                .pop()
                .map(|x| x.as_os_str())
                .and_then(|x| x.to_str())
                .map(|x| x.to_string());

            if ancestor.is_none() {
                continue;
            }

            let path_name = ancestor.unwrap();

            // Handle intermidiary directories.
            let mut current_acestor_id = root_id;
            let mut current_ancestor_path = PathBuf::new();
            current_ancestor_path.push(&root_path);
            for ancestor_name in ancestry {
                let display_name = ancestor_name
                    .clone()
                    .as_os_str()
                    .to_string_lossy()
                    .into_owned();
                if let Some(child_key) = slab[current_acestor_id].child_key(&display_name) {
                    current_acestor_id = child_key;
                } else {
                    let new_entry = slab.vacant_entry();
                    let new_id = new_entry.key();
                    current_ancestor_path.push(&display_name);
                    new_entry.insert(Box::new(File {
                        id: new_id,
                        parent: Some(current_acestor_id),
                        display_name: display_name.clone(),
                        path: current_ancestor_path.to_string_lossy().into_owned(),
                        file_type: FileType::Directory,
                        data: TypeSpecficData::Directory(HashMap::new()),
                    }));
                    slab[current_acestor_id].add_child(&display_name, new_id);
                    current_acestor_id = new_id;
                }
            }

            // Finally, insert the node.
            let new_entry = slab.vacant_entry();
            let new_id = new_entry.key();
            new_entry.insert(Box::new(File {
                id: new_id,
                parent: Some(current_acestor_id),
                display_name: path_name.clone(),
                path: path,
                file_type: meta,
                data: data,
            }));
            slab[current_acestor_id].add_child(&path_name, new_id);
        }

        Some(FileTree {
            storage: slab,
            root_id: root_id,
        })
    }

    pub fn get(&self, id: usize) -> &File {
        &self.storage[id]
    }

    pub fn get_root(&self) -> &File {
        self.get(self.root_id)
    }

    pub fn get_parent(&self, file: &File) -> Option<&File> {
        file.parent.map(|id| self.get(id))
    }
}

#[cfg(test)]
mod test {
    use super::{FileTree, FileType, TypeSpecficData};

    #[test]
    fn tree_construction() {
        let tree = FileTree::new(
            ".",
            vec![
                ("a".to_string(), FileType::File),
                ("b/c/d".to_string(), FileType::File),
            ],
        )
        .unwrap();

        let root = tree.get(tree.root_id);
        assert!(root.is_dir());
        if let TypeSpecficData::Directory(root_chilren) = &root.data {
            assert_eq!(root_chilren.len(), 2);
            let a_id = root_chilren.get("a").expect("a exists");
            let a = tree.get(*a_id);
            assert!(a.is_file());
            let b_id = root_chilren.get("b").expect("b exists");
            let b = tree.get(*b_id);
            assert!(b.is_dir());
            if let TypeSpecficData::Directory(b_children) = &b.data {
                assert_eq!(b_children.len(), 1);
                let c_id = b_children.get("c").expect("c exists");
                let c = tree.get(*c_id);
                assert!(c.is_dir());
                if let TypeSpecficData::Directory(c_children) = &c.data {
                    assert_eq!(c_children.len(), 1);
                    let d_id = c_children.get("d").expect("d exists");
                    let d = tree.get(*d_id);
                    assert!(d.is_file());
                }
            }
        }
    }
}
