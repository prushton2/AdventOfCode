use std::boxed::Box;
use std::fs;

fn main() {
    let binding = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut wordsearch: Vec<&str> = binding.split('\n').collect();
    wordsearch.pop();

    let mut xmascount = 0;

    let patterns: [[[char; 3]; 3]; 4] = [
        [
            ['M', '_', 'M'], 
            ['_', 'A', '_'], 
            ['S', '_', 'S']
        ],
        [
            ['S', '_', 'M'], 
            ['_', 'A', '_'], 
            ['S', '_', 'M']
        ],
        [
            ['M', '_', 'S'], 
            ['_', 'A', '_'], 
            ['M', '_', 'S']
        ],
        [
            ['S', '_', 'S'], 
            ['_', 'A', '_'], 
            ['M', '_', 'M']
        ],
    ];

    let line_len = wordsearch[0].len() - 2;
    for x in 0..(wordsearch.len() - 2) {
        for y in 0..line_len {
            if match_patterns(x, y, &patterns, &wordsearch) {
                xmascount += 1;
            }
        }
    }

    println!("{}", xmascount);
}

fn match_patterns(x: usize, y: usize, patterns: &[[[char; 3]; 3]; 4], wordsearch: &Vec<&str>) -> bool {
    for pattern in patterns {

        let mut matches = true;
        
        for o_y in 0..pattern.len(){
            if !matches {
                continue
            }
            let row = pattern[o_y];
            
            for o_x in 0..row.len() {
                let pattern_char = row[o_x];
                let source_char = wordsearch[y+o_y].chars().nth(x+o_x).unwrap();

                if pattern_char != source_char && pattern_char != '_' {
                    matches = false;
                    break;
                }
            }
        }

        if matches {
            return true
        }
    }

    return false
}