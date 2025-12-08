use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut cursors: [usize; 5] = [0; 5];
    let lines: Vec<Vec<char>> = file.split('\n').map(|c| c.chars().collect::<Vec<char>>()).collect();

    let mut total_sum = 0;
    let mut done = false;

    while !done {

        let mut digits: Vec<String> = vec![];
        let mut op: char = '_';
    
        for cursor_id in 0..cursors.len() {
            let mut cursor = cursors[cursor_id];
            let line = &lines[cursor_id];
    
            let mut digit: String = "".to_owned();
    
            while line[cursor] == ' ' {
                digit.push(' ');

                cursor += 1;
                if cursor >= line.len() {
                    done = true;
                    break
                }
            }
    
            while line[cursor].is_digit(10) {
                digit.push(line[cursor]);
                
                cursor += 1;
                if cursor >= line.len() {
                    done = true;
                    break
                }
            }
    
            if cursor < line.len() && vec!['+', '*'].contains(&line[cursor]) {
                op = line[cursor];
                
                cursor += 1;
                if cursor >= line.len() {
                    done = true;
                    break
                }
                cursors[cursor_id] = cursor;
                continue
            }
            
            digits.push(digit);

            if cursor+1 < line.len() && line[cursor] == ' ' {
                cursor += 1;
            }
            
            cursors[cursor_id] = cursor;
        }

        let max = {
            let mut m = 0;
            for i in cursors {
                if i > m {
                    m = i;
                }
            }
            m
        };

        for i in 0..cursors.len() {
            cursors[i] = max
        }

        
        println!("{:?}: {}", digits, op);
        println!("{:?}", cursors);

        // reform digits

        let mut new_digits: Vec<i64> = vec![];

        let max_len = {
            let mut ml = 0;
            for i in &digits {
                if i.len() > ml {
                    ml = i.len();
                }
            }
            ml
        };

        for cursor in 0..max_len {
            let mut new_digit: i64 = 0;
            for old_digit in &digits {
                let old_digit_chars = old_digit.chars().collect::<Vec<char>>();
                if cursor >= old_digit.len() || old_digit_chars[cursor] == ' ' {
                    continue
                }
                new_digit *= 10;
                new_digit += old_digit_chars[cursor].to_digit(10).unwrap() as i64;
            }
            new_digits.push(new_digit);
        }


        print!("{:?}: {} = ", new_digits, op);

        total_sum += match op {
            '*' => {
                let mut d = 1;
                for i in new_digits {
                    d *= i
                }
                println!("{}\n", d);
                d
            }
            '+' => {
                let mut d = 0;
                for i in new_digits {
                    d += i
                }
                println!("{}\n", d);
                d
            }
            _ => {0}
        }
        
    }

    println!("{}", total_sum)

}
