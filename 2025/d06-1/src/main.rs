use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut cursors: [usize; 5] = [0; 5];
    let lines: Vec<Vec<char>> = file.split('\n').map(|c| c.chars().collect::<Vec<char>>()).collect();

    let mut total_sum = 0;
    let mut done = false;

    while !done {

        let mut digits: Vec<i64> = vec![];
        let mut op: char = '_';
    
        for cursor_id in 0..cursors.len() {
            let mut cursor = cursors[cursor_id];
            let line = &lines[cursor_id];
    
            let mut digit: i64 = 0;
    
            while line[cursor] == ' ' {
                
                cursor += 1;
                if cursor >= line.len() {
                    done = true;
                    break
                }
            }
    
            while line[cursor].is_digit(10) {
                digit *= 10;
                digit += line[cursor].to_digit(10).unwrap() as i64;
                
                cursor += 1;
                if cursor >= line.len() {
                    done = true;
                    break
                }
            }
    
            if digit != 0 {
                digits.push(digit);
            }
    
            if cursor < line.len() && vec!['+', '*'].contains(&line[cursor]) {
                op = line[cursor];
                
                cursor += 1;
                if cursor >= line.len() {
                    done = true;
                    break
                }
            }
            cursors[cursor_id] = cursor;
        }
        print!("{:?}: {} = ", digits, op);

        total_sum += match op {
            '*' => {
                let mut d = 1;
                for i in digits {
                    d *= i
                }
                println!("{}", d);
                d
            }
            '+' => {
                let mut d = 0;
                for i in digits {
                    d += i
                }
                println!("{}", d);
                d
            }
            _ => {0}
        }
        
    }

    println!("{}", total_sum)

}
