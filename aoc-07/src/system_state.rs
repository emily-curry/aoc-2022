use crate::system_command::{ListNode, SystemCommand};
use crate::system_directory::SystemDirectory;
use crate::system_node::SystemNode;

pub struct SystemState {
    pub root: SystemDirectory,
    cwd: Vec<String>,
}

impl SystemState {
    pub fn run(&mut self, command: SystemCommand) {
        match command {
            SystemCommand::ChangeDirectory(dir) => self.cd(dir),
            SystemCommand::List(res) => self.ls(res),
        }
    }

    fn cd(&mut self, dir: String) {
        if dir == ".." {
            self.cwd.pop();
        } else if dir == "/" {
            self.cwd.clear();
        } else {
            self.cwd.push(dir)
        }
    }

    fn ls(&mut self, result: Vec<ListNode>) {
        let dir = self.enter_cwd();
        for node in result {
            let sys_node = SystemNode::from(node);
            dir.contents.insert(sys_node.name().to_string(), sys_node);
        }
    }

    fn enter_cwd(&mut self) -> &mut SystemDirectory {
        let mut dir = &mut self.root;
        for seg in self.cwd.iter() {
            let inner = dir.contents.get_mut(seg).unwrap();
            if let SystemNode::Directory(i) = inner {
                dir = i;
            } else {
                panic!("Path segment {} is not a directory!", seg);
            }
        }
        dir
    }
}

impl Default for SystemState {
    fn default() -> Self {
        let root = SystemDirectory::new("".to_string());
        SystemState {
            root,
            cwd: Vec::new(),
        }
    }
}
