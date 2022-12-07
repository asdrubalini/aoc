use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
};

use itertools::Itertools;

use crate::aoc::Solution;

pub struct Seven;

#[derive(Debug)]
pub enum Command {
    Cd { directory: String },
    Ls { contents: Vec<(String, String)> },
}

impl From<&str> for Command {
    /// tokenize
    fn from(from: &str) -> Self {
        let mut lines = from.lines();

        let command = lines.next().unwrap();
        let mut command_split = command.split_whitespace(); // [ "ls", "sticazzi" ]

        match command_split.next().unwrap() {
            "cd" => {
                let directory = command_split.next().unwrap().to_string();
                Command::Cd { directory }
            }
            "ls" => {
                let contents = lines
                    .map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
                    .map(|(left, right)| (left.to_string(), right.to_string()))
                    .collect_vec();

                Command::Ls { contents }
            }
            _ => panic!("wtf"),
        }
    }
}

#[derive(Debug, Clone)]
struct File {
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    contents: HashMap<OsString, Item>,
}

impl Directory {
    fn create_file(&mut self, name: OsString, size: usize) {
        self.contents.insert(name, Item::File(File { size: size }));
    }

    fn create_directory(&mut self, name: OsString) {
        self.contents.insert(
            name,
            Item::Directory(Directory {
                contents: HashMap::new(),
            }),
        );
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    File(File),
    Directory(Directory),
}

impl Item {
    fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    fn is_directory(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    fn as_file_mut(&mut self) -> &mut File {
        if let Item::File(c) = self {
            c
        } else {
            panic!("not a file");
        }
    }

    fn as_directory_mut(&mut self) -> &mut Directory {
        if let Item::Directory(d) = self {
            d
        } else {
            panic!("not a directory");
        }
    }

    fn as_file(&self) -> &File {
        if let Item::File(c) = self {
            c
        } else {
            panic!("not a file");
        }
    }

    fn as_directory(&self) -> &Directory {
        if let Item::Directory(d) = self {
            d
        } else {
            panic!("not a directory");
        }
    }

    /// recursively (if directory) compute full size
    fn size(&self) -> usize {
        match self {
            Item::File(f) => f.size,
            Item::Directory(d) => d.contents.values().map(|item| item.size()).sum(),
        }
    }

    fn find_directories_recursive(&self) -> Vec<&Directory> {
        if self.is_file() {
            return vec![];
        }

        let mut directories = self
            .as_directory()
            .contents
            .values()
            .map(|item| item.find_directories_recursive())
            .flatten()
            .collect_vec();

        directories.push(self.as_directory());

        directories
    }
}

#[derive(Debug)]
pub struct Tree {
    root: Item,
}

impl Default for Tree {
    /// just /
    fn default() -> Self {
        Self {
            root: Item::Directory(Directory {
                contents: HashMap::default(),
            }),
        }
    }
}

impl Tree {
    /// get a mut reference to the item at path
    fn get_item_mut(&mut self, path: &Path) -> &mut Item {
        // start in root
        let mut current_item = &mut self.root;

        // traverse the paths (skip /)
        for directory in path.iter().skip(1) {
            let current_item_dir = current_item.as_directory_mut();

            current_item = current_item_dir
                .contents
                .get_mut(&directory.to_owned())
                .expect(&format!("cannot find directory {directory:?}"));
        }

        current_item
    }

    fn find_all_directories(&self) -> Vec<&Directory> {
        self.root.find_directories_recursive()
    }
}

impl From<&[Command]> for Tree {
    fn from(commands: &[Command]) -> Self {
        let mut tree = Tree::default();
        let mut pwd = PathBuf::from("/");

        for command in commands {
            match command {
                // manipulate the current path (pwd)
                Command::Cd { directory } => match directory.as_str() {
                    ".." => {
                        pwd.pop();
                    }
                    "/" => pwd = PathBuf::from("/"),
                    _ => pwd.push(directory),
                },

                Command::Ls { contents } => {
                    let current_directory = tree.get_item_mut(&pwd).as_directory_mut();

                    for (left, right) in contents {
                        let name = OsString::from(right);

                        if left == "dir" {
                            current_directory.create_directory(name);
                        } else {
                            let size: usize = left.parse().unwrap();
                            current_directory.create_file(name, size)
                        }
                    }
                }
            }
        }

        tree
    }
}

impl Solution for Seven {
    type Output = usize;
    type Parsed = Tree;

    fn input() -> &'static str {
        include_str!("../inputs/7.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        let commands = input
            .split("$ ")
            .filter(|cmd| !cmd.is_empty())
            .map(Command::from)
            .collect_vec();

        Tree::from(commands.as_slice())
    }

    fn solve_first(parsed: &Self::Parsed) -> Self::Output {
        let directories = parsed
            .find_all_directories()
            .into_iter()
            .map(ToOwned::to_owned)
            .map(Item::Directory);

        //for dir in directories {
        //println!("{}", dir.size());
        //}

        directories
            .map(|d| {
                println!("{:?}", d);
                d.size()
            })
            .filter(|size| *size <= 100000)
            .sum()
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1427048, 0)
    }
}
