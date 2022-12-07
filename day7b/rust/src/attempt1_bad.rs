use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

const FILE_PATH: &str = "../input.txt";

#[derive(Debug, Eq, PartialEq, Clone)]
struct DirData {
    items: HashMap<String, Node>,
    parent: Option<Box<Node>>,
}

impl DirData {
    fn add_item(&mut self, name: String, node: Node) {
        self.items.insert(name, node);
    }
}

impl Hash for DirData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        for item in &self.items {
            item.hash(state)
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct FileData {
    size: u32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Node {
    Dir(DirData),
    File(FileData),
}

fn main() {
    let Ok(file_content) = fs::read_to_string(FILE_PATH) else {
        panic!("Can't read file {FILE_PATH}");
    };

    let mut root = Node::Dir(DirData {
        items: HashMap::new(),
        parent: None,
    });
    let mut node_pointer = Box::new(&root);

    let command_groups: Vec<Vec<Vec<&str>>> = file_content
        .split("$ ")
        .skip(1)
        .map(|command_group| {
            command_group
                .lines()
                .map(|line| line.split(" ").collect())
                .collect()
        })
        .collect();

    // println!("{:?}", command_groups);
    // println!();

    for command_group in &command_groups {
        let command = &command_group[0];

        match command[0] {
            "cd" => match command[1] {
                "/" => {
                    node_pointer = Box::new(&root);
                }
                ".." => {
                    if let Node::Dir(dir_data) = *node_pointer {
                        let parent_ref = dir_data.parent.as_ref();
                        let parent_node = &**parent_ref.unwrap();
                        node_pointer = Box::new(parent_node);
                    } else {
                        panic!("Invalid state: node_pointer inside file");
                    }
                }
                name => {
                    let next_dir = *node_pointer.as_ref();
                    if let Node::Dir(dir_data) = next_dir {
                        let dir_ref = &dir_data.items.get(name);
                        let parent_node = &*dir_ref.unwrap();
                        node_pointer = Box::new(parent_node);
                    }
                }
            },
            "ls" => {
                let output = &command_group[0..];
                println!("{:?}", output);

                let node_p = node_pointer.clone().as_ref();
                if let Node::Dir(mut dir_data) = node_pointer.as_mut() {
                    for line in output {
                        match line[..] {
                            ["dir", name] => {
                                dir_data.add_item(
                                    String::from(name),
                                    Node::Dir(DirData {
                                        parent: Some(Box::new(*node_p)),
                                        items: HashMap::new(),
                                    }),
                                );
                            }
                            [size, name] => (),
                            _ => panic!(""),
                        }
                    }
                }
            }
            &_ => {}
        }
    }

    println!();
    println!("{:?}", root);
}
