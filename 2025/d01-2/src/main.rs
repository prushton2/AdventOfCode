use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut head: i32 = 50;
    let mut zeroCount = 0;

    for line in file.split('\n') {
        if line == "" {
            continue;
        }
         
        let amount = line[1..].parse::<i32>().unwrap();
        let sign = {
            if line.chars().nth(0) == Some('L') {
                -1
            } else {
                1
            }
        };

        for _ in 0..amount {
            head += sign;
            head %= 100;
            if head == 0 {
                zeroCount += 1;
            }
        }
    }
    println!("{}", zeroCount)
}
