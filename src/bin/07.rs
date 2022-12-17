use core::panic;
use regex::Regex;

#[derive(Debug)]
enum Node {
    D(Dir),
    F(File),
}

#[derive(Debug)]
struct File {
    size: u32,
}

#[derive(Debug)]
struct Dir {
    name: String,
    parent: usize,
    children: Vec<usize>,
}

struct Fs {
    root: usize,
    current_dir: usize,
    nodes: Vec<Node>,
}

impl Fs {
    fn new() -> Self {
        Self {
            root: 0,
            current_dir: 0,
            nodes: vec![Node::D(Dir::new(0, "/"))],
        }
    }

    fn find(&self, dirname: &str) -> Option<&Dir> {
        let Some((_, Node::D(d))) =&self.nodes.iter().enumerate().find(|(_, n)| match n {
            Node::D(d) => {
                if d.name == dirname {
                    true
                } else {
                    false
                }
            }
            Node::F(_) => false,
        }) else {
            return None;
        };

        Some(d)
    }

    fn get_dir(&self, dir: usize) -> Option<&Dir> {
        let Node::D(d) = &self.nodes[dir] else {
            return None;
        };
        Some(d)
    }

    fn cd(&mut self, dirname: &str) {
        if dirname == ".." {
            let Node::D(d) = &self.nodes[self.current_dir] else {
                panic!("bruh something happened i guess")
            };
            self.current_dir = d.parent;
            return;
        }

        if let Some(index) = self
            .get_dir(self.current_dir)
            .unwrap()
            .children
            .iter()
            .find(|i| match &self.nodes[**i] {
                Node::D(d) => {
                    if d.name == dirname {
                        true
                    } else {
                        false
                    }
                }
                Node::F(_) => false,
            })
        {
            self.current_dir = *index;
            return;
        }

        self.current_dir = self.create_dir(dirname);
    }

    fn create_dir(&mut self, dirname: &str) -> usize {
        let dir = Dir::new(self.current_dir, dirname);
        self.add_node(Node::D(dir))
    }

    fn add_node(&mut self, node: Node) -> usize {
        let new_idx = self.nodes.len();
        self.nodes.push(node);

        let Node::D(current_dir) = &mut self.nodes[self.current_dir] else {
            panic!()
        };

        current_dir.children.push(new_idx);
        new_idx
    }

    fn from_input(input: &str) -> Self {
        let cmd_reg = Regex::new(r"\B\$\s+(\w+)\b(?:\s+(.+))?").unwrap();
        let ls_out_reg = Regex::new(r"\b(\d+|dir)\s([\w][\w.]*)\b").unwrap();

        let mut fs = Fs::new();

        let mut current_cmd = String::new();
        let mut arg = String::new();

        for line in input.lines() {
            if cmd_reg.is_match(line) {
                let cmd = cmd_reg.captures(line).unwrap();
                current_cmd = cmd[1].to_string();

                if let Some(a) = cmd.get(2) {
                    arg = a.as_str().to_string();
                };
            }

            match current_cmd.as_str() {
                "cd" => fs.cd(&arg),
                "ls" => {
                    if cmd_reg.is_match(line) {
                        continue;
                    }

                    let c = ls_out_reg.captures(line).unwrap();

                    if &c[1] == "dir" {
                        fs.create_dir(&c[2]);
                    } else {
                        fs.add_node(Node::F(File {
                            size: c[1].parse::<u32>().expect("cant file size parse as u32"),
                        }));
                    }
                }
                _ => {
                    panic!("unknown command")
                }
            }
        }

        fs
    }
}

impl Dir {
    fn new(parent: usize, dirname: &str) -> Self {
        Self {
            name: dirname.to_string(),
            children: vec![],
            parent,
        }
    }
}

fn find_dir_with_size(fs: &Fs, dir: usize, total: &mut Box<u32>) -> u32 {
    let node = fs.get_dir(dir).unwrap();

    let mut size = 0;

    for n in node.children.iter() {
        match &fs.nodes[*n] {
            Node::D(_) => size += find_dir_with_size(fs, *n, total),
            Node::F(f) => size += f.size,
        }
    }

    if size <= 100000 {
        **total = **total + size;
    }

    size
}

pub fn part_one(input: &str) -> Option<u32> {
    let fs = Fs::from_input(input);
    let mut total = Box::new(0);
    find_dir_with_size(&fs, fs.root, &mut total);
    println!("{:?}", fs.find("nzpvt"));

    Some(*total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437 + 2 * 111));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
