use crate::system_command::ListNode;
use crate::system_directory::SystemDirectory;

pub enum SystemNode {
    Directory(SystemDirectory),
    File(String, usize),
}

impl SystemNode {
    pub fn name(&self) -> &str {
        match self {
            SystemNode::Directory(dir) => dir.name.as_str(),
            SystemNode::File(name, _) => name.as_str(),
        }
    }
}

impl From<ListNode> for SystemNode {
    fn from(input: ListNode) -> Self {
        match input {
            ListNode::Directory(name) => SystemNode::Directory(SystemDirectory::new(name)),
            ListNode::File(name, size) => SystemNode::File(name, size),
        }
    }
}
