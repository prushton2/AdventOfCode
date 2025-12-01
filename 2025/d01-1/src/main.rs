use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut head: i32 = 50;
    let mut zeroCount = 0;

    for line in file.split('\n') {
        if line == "" {
            continue;
        }
         
        let mut amount = line[1..].parse::<i32>().unwrap();

        if line.chars().nth(0) == Some('L') {
            amount *= -1;
        }

        head += amount;

        while head < 0 {
            head += 100;
        }

        head %= 100;

        if head == 0 {
            zeroCount += 1;
        }
    }
    println!("{}", zeroCount)
}
