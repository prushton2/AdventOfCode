use std::fs;
use std::collections::HashMap;

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Bad file read");
    let array: Vec<&str> = file.split(&['\n', ' '][..]).collect();

    let mut lhs: Vec<u64> = vec![];
    let mut rhs: HashMap<u64, u64> = [].into();

    let mut scores: u64 = 0;

    let mut i = 0;
    for item in array {
        if item == "" {
            continue;
        }

        if i % 2 == 0 {
            lhs.push(item.parse::<u64>().unwrap());
        } else {
            match rhs.insert(item.parse::<u64>().unwrap(), 1) {
                Some(n) => {rhs.insert(item.parse::<u64>().unwrap(), n+1);},
                None => {}
            }
        }
        i += 1;
    }

    for item in lhs {
        let score: u64 = item * match rhs.get(&item) {Some(n) => n, None => &0};
        scores += score;
    }

    println!("{}", scores)
    
}
