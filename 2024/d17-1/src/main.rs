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
    fn read(&self, registers: &HashMap<char, i64>) -> i64 {
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
    fn write(&self, value: i64, registers: &mut HashMap<char, i64>) -> bool {
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

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut registers: HashMap<char, i64> = [].into();

    let lines = file.split('\n').collect::<Vec<&str>>();
    
    registers.insert('A', lines[0].split(": ").nth(1).unwrap().parse().unwrap());
    registers.insert('B', lines[1].split(": ").nth(1).unwrap().parse().unwrap());
    registers.insert('C', lines[2].split(": ").nth(1).unwrap().parse().unwrap());
    
    let instruction_mem: Vec<u8> = lines[4].split(": ").nth(1).unwrap().split(',').map(|c| c.parse::<u8>().unwrap()).collect();
    let mut ipointer = 0;
    let mut out: Vec<i64> = vec![];

    while ipointer < instruction_mem.len() {
        let opcode = Operation::from_u8(instruction_mem[ipointer]).unwrap();
        let operand: i64 = instruction_mem[ipointer+1] as i64;
        let combo_operand = ComboOperand::from_u8(instruction_mem[ipointer+1]).unwrap();

        println!("{:?} {:?}, {:?}", opcode, operand, combo_operand.read(&registers));

        match opcode {
            Operation::adv => {
                let res: i64 = registers.get(&'A').unwrap() / &(2 as i64).pow(combo_operand.read(&registers) as u32);
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
                let res: i64 = registers.get(&'A').unwrap() / &(2 as i64).pow(combo_operand.read(&registers) as u32);
                ComboOperand::rB.write(res, &mut registers);
                ipointer += 2;
            },
            Operation::cdv => {
                let res: i64 = registers.get(&'A').unwrap() / &(2 as i64).pow(combo_operand.read(&registers) as u32);
                ComboOperand::rC.write(res, &mut registers);
                ipointer += 2;
            },
        }
    }
    println!("{}", out.into_iter().map(|c| format!("{c}")).collect::<Vec<String>>().join(","));
}
