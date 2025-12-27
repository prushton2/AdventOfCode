use std::{collections::HashMap, fs};

#[derive(PartialEq)]
enum NodeType {
    Continue,
    End
}

// This is the same idea as a word search, but we need to be more exhaustive.
struct Node {
    color: char,
    node_type: NodeType,
    children: HashMap<char, Box<Node>>
}

impl Node {
    fn new() -> Self {
        return Self {
            color: '_',
            node_type: NodeType::Continue,
            children: [].into()
        }
    }

    // goofy wrapper
    fn insert(&mut self, str: &str) {
        self.ins(&str.chars().collect::<Vec<char>>());
    }

    // crawl down the word, insert it whereever, then set the last character as an end node
    fn ins(&mut self, str: &[char]) {

        if str.len() == 0 {
            self.node_type = NodeType::End;
            return
        }

        let child = match self.children.get_mut(&str[0]) {
            Some(t) => t,
            None => {
                self.children.insert(str[0], Box::new(Self::new()));
                self.children.get_mut(&str[0]).unwrap()
            }
        };

        child.color = str[0];
        child.ins(&str[1..]);
    }

    fn crawl<'a>(&self, str: &'a str) -> bool {
        let mut node = self;
        let chars = str.chars().collect::<Vec<char>>();

        // loop over each character, climbing the tree as we go
        for i in 0..str.len() {
            node = match node.children.get(&chars[i]) {
                Some(t) => t,
                None => return false // here we dont find any pattern that fits the text, so return false
            };

            // this is the end of a pattern that matches the text, so we start a new pattern recursively
            if node.node_type == NodeType::End {
                let remainder = &str[i+1..];
                
                if remainder.len() == 0 {
                    return true;
                }

                match self.crawl(remainder) {
                    true => return true,
                    false => continue // since we didnt find a match at the current pattern, we explore the next pattern and try again
                };
                
            }
        }

        return false
    }
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");

    let mut towels = Node::new();
    let mut patterns: Vec<String> = vec![];

    for s in file.split("\n\n").nth(0).unwrap().split(", ") {
        towels.insert(s);
    }

    for p in file.split("\n\n").nth(1).unwrap().split("\n") {
        patterns.push(p.to_owned());
    }

    let mut sum = 0;
    
    for pattern in patterns {
        let res = towels.crawl(&pattern);
        
        if res {
            sum += 1;
        }

        println!("{}: {}", pattern, res);
    }
    println!("{sum}");
}
