use std::{collections::HashMap, fs};

#[derive(Debug)]
enum Operation {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7
}

impl Operation {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Operation::adv),
            1 => Some(Operation::bxl),
            2 => Some(Operation::bst),
            3 => Some(Operation::jnz),
            4 => Some(Operation::bxc),
            5 => Some(Operation::out),
            6 => Some(Operation::bdv),
            7 => Some(Operation::cdv),
            _ => None
        }
    }
}

#[derive(Debug)]
enum ComboOperand {
    d0 = 0,
    d1 = 1,
    d2 = 2,
    d3 = 3,
    rA = 4,
    rB = 5,
    rC = 6,
    d7 = 7
}

impl ComboOperand {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ComboOperand::d0),
            1 => Some(ComboOperand::d1),
            2 => Some(ComboOperand::d2),
            3 => Some(ComboOperand::d3),
            4 => Some(ComboOperand::rA),
            5 => Some(ComboOperand::rB),
            6 => Some(ComboOperand::rC),
            7 => Some(ComboOperand::d7),
            _ => None
        }
    }
    fn read(&self, registers: &HashMap<char, u64>) -> u64 {
        match self {
            ComboOperand::d0 => 0,
            ComboOperand::d1 => 1,
            ComboOperand::d2 => 2,
            ComboOperand::d3 => 3,
            ComboOperand::rA => *registers.get(&'A').unwrap(),
            ComboOperand::rB => *registers.get(&'B').unwrap(),
            ComboOperand::rC => *registers.get(&'C').unwrap(),
            ComboOperand::d7 => panic!("Invalid Combo Operand")
        }
    }
    fn write(&self, value: u64, registers: &mut HashMap<char, u64>) -> bool {
        match self {
            ComboOperand::d0 => false,
            ComboOperand::d1 => false,
            ComboOperand::d2 => false,
            ComboOperand::d3 => false,
            ComboOperand::rA => registers.insert('A', value).is_some(),
            ComboOperand::rB => registers.insert('B', value).is_some(),
            ComboOperand::rC => registers.insert('C', value).is_some(),
            ComboOperand::d7 => panic!("Invalid Combo Operand")
        }
    }
}

fn read_program() -> Vec<u64> {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let lines = file.split('\n').collect::<Vec<&str>>();
    return lines[4].split(": ").nth(1).unwrap().split(',').map(|c| c.parse::<u64>().unwrap()).collect();
}

fn simulate(register_a: u64) -> Vec<u64> {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut registers: HashMap<char, u64> = [].into();

    let lines = file.split('\n').collect::<Vec<&str>>();
    
    // registers.insert('A', lines[0].split(": ").nth(1).unwrap().parse().unwrap());
    registers.insert('A', register_a);
    registers.insert('B', lines[1].split(": ").nth(1).unwrap().parse().unwrap());
    registers.insert('C', lines[2].split(": ").nth(1).unwrap().parse().unwrap());
    
    let instruction_mem: Vec<u8> = lines[4].split(": ").nth(1).unwrap().split(',').map(|c| c.parse::<u8>().unwrap()).collect();
    let mut ipointer = 0;
    let mut out: Vec<u64> = vec![];

    while ipointer < instruction_mem.len() {
        let opcode = Operation::from_u8(instruction_mem[ipointer]).unwrap();
        let operand: u64 = instruction_mem[ipointer+1] as u64;
        let combo_operand = ComboOperand::from_u8(instruction_mem[ipointer+1]).unwrap();

        // println!("{:?} {:?}, {:?}", opcode, operand, combo_operand.read(&registers));

        match opcode {
            Operation::adv => {
                let res: u64 = registers.get(&'A').unwrap() / &(2 as u64).pow(combo_operand.read(&registers) as u32);
                ComboOperand::rA.write(res, &mut registers);
                ipointer += 2;
            },
            Operation::bxl => {
                let bxr = ComboOperand::rB.read(&registers) ^ operand;
                ComboOperand::rB.write(bxr, &mut registers);
                ipointer += 2;
            },
            Operation::bst => {
                let res = combo_operand.read(&registers) % 8;
                ComboOperand::rB.write(res, &mut registers);
                ipointer += 2;
            },
            Operation::jnz => {
                if ComboOperand::rA.read(&registers) != 0 {
                    ipointer = operand as usize;
                    // println!("Looping {}", ComboOperand::rA.read(&registers));
                } else {
                    ipointer += 2;
                }
            },
            Operation::bxc => {
                let res = ComboOperand::rB.read(&registers) ^ ComboOperand::rC.read(&registers);
                ComboOperand::rB.write(res, &mut registers);
                ipointer += 2;
            },
            Operation::out => {
                let res = combo_operand.read(&registers) % 8;
                out.push(res);
                ipointer += 2;
            },
            Operation::bdv => {
                let res: u64 = registers.get(&'A').unwrap() / &(2 as u64).pow(combo_operand.read(&registers) as u32);
                ComboOperand::rB.write(res, &mut registers);
                ipointer += 2;
            },
            Operation::cdv => {
                let res: u64 = registers.get(&'A').unwrap() / &(2 as u64).pow(combo_operand.read(&registers) as u32);
                ComboOperand::rC.write(res, &mut registers);
                ipointer += 2;
            },
        }
    }
    return out;
}

fn main() {
    let correct_program = vec![2,4,1,1,7,5,0,3,1,4,4,5,5,5,3,0];

    println!("Sim: {}", simulate(202322348616234).iter().map(|c| format!("{c}")).collect::<Vec<String>>().join(","));
    match recurse_in_reverse(&correct_program, 0, correct_program.len()-1) {
        Some(t) => {
            let check = simulate(t);
            println!("Found answer: {t}\nChecking:\n{:?} (correct) \n{:?} (calculated)", correct_program, check);
        },
        None => {
            println!("None")
        }
    }
}

fn recurse_in_reverse(correct_program: &Vec<u64>, register_a: u64, depth: usize) -> Option<u64> {

    left_pad(correct_program.len(), depth);
    println!("{}  | Finding digit {}", depth, correct_program[depth]);

    
    // what digit of the output we are solving for
    
    for i in 0..8 {
        let mut reg_a = register_a << 3;
        reg_a |= i;

        let out = simulate(reg_a);
        if out[0] != correct_program[depth] {
            continue;
        }
        
        left_pad(correct_program.len(), depth);
        println!("{depth}  | Found input {} prints out digit {}", i, out[0]);

        left_pad(correct_program.len(), depth);
        println!("{}  | out: {:?}", depth, out);

        if depth == 0 {
            return Some(reg_a);
        }

        match recurse_in_reverse(correct_program, reg_a, depth-1) {
            Some(t) => return Some(t),
            None => continue
        }
    }

    left_pad(correct_program.len(), depth);
    println!("{}  | No digits found to print {}", depth, correct_program[depth]);

    return None;
}

fn left_pad(lim: usize, digit: usize) {
    for _ in 0..lim-digit {
        print!("  ");
    }
}
/*
    My assembly translates to:

    0: BST: B = A%8
    1: BXL: B = B^1
    2: CDV: C = A/2^B
    3: ADV: A = A/8
    4: BXL: B = B^4
    5: BXC: B = B^C
    6: OUT: PUSH(B%8)
    7: JNZ: IF A != 0 GOTO 0
    8: HALT


    The first thing i realized is the number of iterations is equal to log_8(A)+1.
    This means, to reach my goal of 14 numbers, i need to be within the range of 
    8^13 and 8^14. This doesnt help much, but its a start
    Edit: 8^14 is not the upper limit. I dont know what it is.

    I appear to be "chunking" off a section of A every iteration. Interesting.

    The chunks cant be treated as pure functions since A is the divisor put into instruction 2

    I printed out the outputs from 0..3362075 to look for patterns in the output, and heres what i found:

    the final character order is 5,5,7,6,1,0,3,2
    after the output, it loops but prints 8 of each number at the end (the two 5s merge). This repeats indefinitely, going to 64, 
    then 512, etc. This means that I can isolate the range of valid numbers further.
    ie: the final instructions for [9, 64) is 5,5,5,5,5,5,5,5, 7,7,7,7,7,7,7,7, 6,6,6,6,6,6,6,6, 1,1,1,1,1,1,1,1, 0,0,0,0,0,0,0,0, 3,3,3,3,3,3,3,3, 2,2,2,2,2,2,2,2,
    This means if 
        f(x) = x / (x^(floor(log_8(x)+1)))
    then
        last_instruction = 0 if 0.625 <= f(x) < 0.75


    New Plan:

    When calculating the last digit, only the last 3 bits matter. When you calculate the second to
    last digit, the last 6 bits matter, etc. This means if i know all possible combinations that end in
    a 0, i can try for combinations that end in 3, 0
*/

//## Old Code ##//

/*
fn main2() {
    let mut chunks: Vec<i64> = vec![7];
    let program = read_program();

    println!("{:?}", find_digit(&vec![], &program, 0));
}


fn find_digit(chunks: &Vec<i64>, program: &Vec<i64>, digit: usize) -> Option<Vec<i64>> {
    if digit >= program.len() {
        return Some(chunks.clone());
    }

    for i in 0..8 {
        let new_chunk = i as i64;
        println!("  Char {i}");

        let value = simulate(form_register(&chunks, new_chunk));

        println!("{:?}\n{:?}", value, program);

        if value.len() >= digit+1 && value[digit] == program[digit] {
            println!("  Same digit");
            
            let mut new_chunks = chunks.clone();
            new_chunks.push(new_chunk);

            match find_digit(chunks, program, digit+1) {
                Some(t) => return Some(t),
                None => {}
            }

        }
    }

    return None
}


fn next_valid_attempt(attempt: u64) -> u64 {
    let pattern_start = index_last_item(attempt, 5);
    

    return pattern_start
}

fn index_last_item(attempt: u64, desired_index: u64) -> u64 {
    if g(attempt+1) == desired_index {
        return attempt+1;
    }
    if g(attempt) < desired_index {
        return (8 as u64).pow(c(attempt)-1)*desired_index
    }
    return (8 as u64).pow(c(attempt))*desired_index
}

fn g(x: u64) -> u64 {
    8*x / (8 as u64).pow(c(x))
}

fn c(x: u64) -> u32 {
    if x < 1 {
        return 0;
    }
    x.ilog(8)+1
}

fn recurse_in_reverse(correct_program: &Vec<u64>, register_a: u64, depth: usize) -> Option<u64> {

    if chunks.len() == correct_program.len() {
        let mut register = 0;
        for chunk in chunks {
            register |= chunk;
            register = register << 3;
        }
        register = register >> 3;
        println!("Reached end, returning {} constructed from chunks {:?}", register, chunks);
        return Some(register);
    }

    let digit = correct_program.len()-1-chunks.len();

    left_pad(correct_program.len(), digit);
    println!("{}  | Finding digit {}", digit, correct_program[digit]);


    // what digit of the output we are solving for

    for i in 0..8 {
        let out = simulate(form_register(correct_program.len(), &chunks, i));
        if out[0] != correct_program[digit] {
            continue;
        }
        
        left_pad(correct_program.len(), digit);
        println!("{}  | Found input {} prints out digit {}", digit, i, out[0]);

        left_pad(correct_program.len(), digit);
        println!("{}  | out: {:?}", digit, out);

        let mut new_chunks = vec![i];
        new_chunks.extend_from_slice(&chunks);

        match recurse_in_reverse(correct_program, &new_chunks) {
            Some(t) => return Some(t),
            None => continue
        }
    }

    left_pad(correct_program.len(), digit);
    println!("{}  | No digits found to print {}", digit, correct_program[digit]);

    return None;
}
*/