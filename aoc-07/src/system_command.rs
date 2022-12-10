pub enum SystemCommand {
    ChangeDirectory(String),
    List(Vec<ListNode>),
}

impl From<&str> for SystemCommand {
    fn from(input: &str) -> Self {
        let mut split = input.split(' ').skip(1);
        match split.next().unwrap() {
            "cd" => SystemCommand::ChangeDirectory(split.next().unwrap().to_string()),
            "ls" => SystemCommand::List(Vec::new()),
            _a => panic!("Unknown command: {}", _a),
        }
    }
}

pub enum ListNode {
    Directory(String),
    File(String, usize),
}

impl From<&str> for ListNode {
    fn from(input: &str) -> Self {
        if input.starts_with("dir") {
            ListNode::Directory(input.split(' ').skip(1).next().unwrap().to_string())
        } else {
            let mut split = input.split(' ');
            let size = split.next().map(|x| x.parse().unwrap()).unwrap();
            let name = split.next().unwrap().to_string();
            ListNode::File(name, size)
        }
    }
}
