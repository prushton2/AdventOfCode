use std::fs;
struct Problem {
    pub result: i64,
    pub operands: Vec<i64>,
}
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut problems: Vec<Problem> = vec![];

    for row in file.split('\n') {
        if row == "" {
            break;
        }

        let row = row.replace(":", "");
        let mut numbers = row.split(' ');
        let mut problem = Problem {
            result: numbers.next().unwrap().parse::<i64>().unwrap(),
            operands: vec![],
        };
        for i in numbers {
            problem.operands.push(i.parse::<i64>().unwrap());
        }

        problems.push(problem);
    }

    let mut sum = 0;

    for problem in problems {
        let combinations = (2 as i64).pow((problem.operands.len() - 1) as u32);
        let mut combination_no = 0;

        // println!("\n\nProblem {}: {:?} {} combinations", problem.result, problem.operands, combinations);

        while combination_no < combinations {
            let mut eval = problem.operands[0];
            let mut iteration = 0;
            // let mut this_combination = combination_no;

            let mut binary_combo = {
                let mut s = "".to_string();
                let mut c = combination_no;
                while c > 0 {
                    s.insert_str(0, format!("{}", c % 2).as_str());
                    c /= 2;
                }
                s
            };

            while binary_combo.len() < problem.operands.len() {
                binary_combo.insert(0, '0');
            }

            // print!("combo {}; {}", binary_combo, eval);

            while iteration < -1 + problem.operands.len() as i64 {
                iteration += 1;
                match binary_combo.chars().nth(iteration as usize) {
                    Some('0') => {
                        eval += problem.operands[iteration as usize];
                        // print!(" +  {}", problem.operands[iteration as usize])
                    }
                    Some('1') => {
                        eval *= problem.operands[iteration as usize];
                        // print!(" *  {}", problem.operands[iteration as usize])
                    }
                    _ => {}
                }
            }

            if eval == problem.result {
                // println!(" Success");
                sum += problem.result;
                break;
            } else {
                // println!(" Failure");
            }

            combination_no += 1;
        }
    }
    println!("{}", sum);
}
