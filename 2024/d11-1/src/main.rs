use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut stones: Vec<i64> = vec![];

    for i in file.split(' ') {
        stones.push(i.parse::<i64>().unwrap());
    }

    for i in 0..25 {
        let mut new_stones: Vec<i64> = vec![];
        for i in 0..stones.len() {
            let stone = stones[i];

            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            if stone.ilog(10)%2 == 1 {
                let chars = 1+stone.ilog(10) as usize;
                let string = format!("{}", stone);

                new_stones.push(string[0..chars/2].parse::<i64>().unwrap());
                new_stones.push(string[chars/2..chars].parse::<i64>().unwrap());
                continue;
            }

            new_stones.push(stone * 2024);
        }
        stones = new_stones;
    }
    println!("{}", stones.len())
}