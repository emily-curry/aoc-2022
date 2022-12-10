use crate::system_node::SystemNode;
use std::collections::BTreeMap;

pub struct SystemDirectory {
    pub name: String,
    pub contents: BTreeMap<String, SystemNode>,
}

impl SystemDirectory {
    pub fn new(name: String) -> Self {
        SystemDirectory {
            name,
            contents: BTreeMap::new(),
        }
    }

    pub fn flat_directories(&self) -> Vec<&SystemDirectory> {
        self.contents
            .values()
            .map(|node| match node {
                SystemNode::Directory(dir) => Some(dir),
                _ => None,
            })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .flat_map(|x| x.flat_directories())
            .chain([self].into_iter())
            .collect()
    }

    pub fn size(&self) -> usize {
        self.contents
            .values()
            .map(|node| match node {
                SystemNode::Directory(dir) => dir.size(),
                SystemNode::File(_, size) => *size,
            })
            .fold(0usize, |acc, val| acc + val)
    }
}
