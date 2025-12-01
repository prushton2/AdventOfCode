use std::boxed::Box;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum NodeType {
    EndOfWord,
    DuringWord,
}
#[derive(Clone)]
struct Node {
    pub nodetype: NodeType,
    pub children: [Option<Box<Node>>; 26],
}

impl Node {
    pub fn new() -> Self {
        return Node {
            nodetype: NodeType::DuringWord,
            children: [const { None }; 26],
        };
    }

    pub fn insert_word(&mut self, word: &str) {
        let letter = match word.chars().nth(0) {
            Some(n) => n,
            None => {
                self.nodetype = NodeType::EndOfWord;
                return;
            }
        };

        let index = (letter as usize) - ('A' as usize);

        match &mut self.children[index] {
            Some(n) => {
                n.insert_word(&word[1..]);
            }
            None => {
                self.children[index] = Some(Box::new(Node::new()));
                self.children[index]
                    .as_mut()
                    .expect("Error inserting new node")
                    .insert_word(&word[1..]);
            }
        }
    }

    pub fn get_letter(&self, c: char) -> &Option<Box<Node>> {
        let index = (c as usize) - ('A' as usize);
        return &self.children[index];
    }
}

fn main() {
    let binding = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut wordsearch: Vec<&str> = binding.split('\n').collect();
    wordsearch.pop();
    let words = ["XMAS"];
    let mut xmascount = 0;

    let mut dict = Node::new();

    for word in words {
        dict.insert_word(word);
    }

    // for word in words {
    //     let mut node = &dict;
    //     let mut i = 0;
    //     loop {
    //         match node.get_letter(word.chars().nth(i).unwrap()) {
    //             Some(n) => node = n.as_ref(),
    //             None => {
    //                 println!("Could not finish word");
    //                 break;
    //             }
    //         }

    //         println!("{}: {:?}", word.chars().nth(i).unwrap(), node.nodetype);
    //         i += 1;
    //     }
    // }

    // return;

    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let line_len = wordsearch[0].len();
    for x in 0..wordsearch.len() as isize {
        for y in 0..line_len as isize {
            for direction_no in 0..directions.len() {
                let direction = directions[direction_no];
                let mut multiplier: isize = 0;
                let mut node = &dict;

                let mut pos: (isize, isize) =
                    (x + direction.0 * multiplier, y + direction.1 * multiplier);

                print!("{}", direction_no);
                loop {
                    let letter = wordsearch[pos.0 as usize].chars().nth(pos.1 as usize).unwrap();

                    print!("{}", letter);

                    let result = match node.get_letter(letter) {
                        Some(n) => n,
                        None => {
                            println!(" -");
                            break;
                        }
                    };

                    if result.as_ref().nodetype == NodeType::EndOfWord {
                        xmascount += 1;
                        println!(" +");
                        break;
                    }

                    node = result.as_ref();

                    multiplier += 1;
                    pos = (x + direction.0 * multiplier, y + direction.1 * multiplier);

                    if pos.0 >= line_len as isize
                        || pos.0 < 0
                        || pos.1 >= wordsearch.len() as isize
                        || pos.1 < 0
                    {
                        println!(" -");
                        break;
                    }
                }
            }
            println!("")
        }
    }

    println!("{}", xmascount);
}
