use std::fs;
struct Instruction {
    pub a: (i64, i64),
    pub b: (i64, i64),
    pub prize: (i64, i64)
}
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");

    let instructions: Vec<Instruction> = {
        let mut v: Vec<Instruction> = vec![];
        for string in file.split("\n\n") {
            let mut instruction: Instruction = Instruction { a: (0, 0), b: (0, 0), prize: (0, 0) };
            for line in string.split("\n") {
                let mut temp = line.split(&[':', ',']);

                match temp.next() {
                    Some("Button A") => {
                        instruction.a = (
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap(), 
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap()
                        )
                    },
                    Some("Button B") => {
                        instruction.b = (
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap(), 
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap()
                        )
                    },
                    Some("Prize") => {
                        instruction.prize = (
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap() + 10000000000000, 
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap() + 10000000000000
                        )
                    }
                    _ => {}
                }
            }
            v.push(instruction);
        }
        v
    };

    let mut cost = 0;

    for instruction in instructions {
        println!("Instruction:\n    A: {:?}\n    B: {:?}\n    P:{:?}", instruction.a, instruction.b, instruction.prize);
        let solution = solve_instruction(&instruction);
        println!("    S: {:?}\n", solution);
        match solution {
            Some(n) => cost += n,
            None => {}
        }
    }

    println!("Cost: {}", cost);
}

// did NOT work at ALL
fn solve_instruction(instruction: &Instruction) -> Option<i64> {

    // cramer's rule or something (idk linear algebra)
    let a_presses = (instruction.prize.0 * instruction.b.1 - instruction.prize.1 * instruction.b.0) / (instruction.a.0 * instruction.b.1 - instruction.a.1 * instruction.b.0);
    let b_presses = (instruction.prize.0 * instruction.a.1 - instruction.prize.1 * instruction.a.0) / (instruction.b.0 * instruction.a.1 - instruction.b.1 * instruction.a.0);

    if (a_presses * instruction.a.0 + b_presses * instruction.b.0, a_presses * instruction.a.1 + b_presses * instruction.b.1) == instruction.prize {
        return Some(a_presses*3+b_presses)
    }
    return None
}
