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
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap(), 
                            temp.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i64>().unwrap()
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
        match solve_instruction(&instruction) {
            Some(n) => cost += n,
            None => {}
        }
    }

    println!("Cost: {}", cost);
}

// did NOT work at ALL
fn solve_instruction(instruction: &Instruction) -> Option<i64> {

    let mut lowest_cost: i64 = i64::MAX;
    let mut a_presses = 0;
    let mut b_presses;
    
    loop {
        b_presses = 0;
        loop {
            let pos = (
                instruction.a.0 * a_presses + instruction.b.0 * b_presses,
                instruction.a.1 * a_presses + instruction.b.1 * b_presses,
            );
            
            if pos.0 == instruction.prize.0 && pos.1 == instruction.prize.1 {
                let cost = a_presses * 3 + b_presses * 1;
                if cost < lowest_cost {
                    lowest_cost = cost
                }
            }
            
            if pos.0 > instruction.prize.0 || pos.1 > instruction.prize.1 {
                break;
            }
            b_presses += 1;
        }
        a_presses += 1;

        if a_presses * instruction.a.0 > instruction.prize.0 || a_presses * instruction.a.1 > instruction.prize.1 {
            break;
        }
    }

    if lowest_cost == i64::MAX {
        return None
    }
    return Some(lowest_cost)
}
