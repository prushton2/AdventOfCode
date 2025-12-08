use std::fs;

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let rows: Vec<Vec<char>> = {
        let mut r: Vec<Vec<char>> = vec![];
        for line in file.split('\n') {
            r.push(line.chars().collect());
        }
        r
    };

    let mut threads: Vec<u64> = vec![0; rows[0].len()];

    for row in &rows {
        let mut new_threads: Vec<u64> = vec![0; rows[0].len()];
        for i in 0..row.len() {
            let char = row[i];
            match char {
                '^' => {
                    new_threads[i-1] += threads[i];
                    new_threads[i+1] += threads[i];
                }
                '.' => {
                    new_threads[i] += threads[i];
                }
                'S' => {
                    new_threads[i] = 1;
                }
                _ => {}
            }
        }
        threads = new_threads;
    }

    let mut sum = 0;
    for i in threads {
        sum += i;
    }
    println!("{sum} threads");
}