use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let batteries: Vec<Vec<u8>> = {
        let mut v: Vec<Vec<u8>> = vec![];
        for line in file.split('\n') {
            if line == "" {
                continue;
            }
            v.push(line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
        }
        v
    };

    let mut total_joltage: u32 = 0;

    for bank in batteries {
        let mut top_2: [usize; 2] = [0, 1];
        for i in 0..bank.len()-1 {
            let battery = bank[i];
            
            if battery > bank[top_2[0]] {
                top_2[0] = i;
                top_2[1] = i+1;
            }
        }

        for i in (top_2[0]+1)..bank.len() {
            let battery = bank[i];
            
            if battery > bank[top_2[1]] {
                top_2[1] = i;
            }
        }

        println!("{}", (bank[top_2[0]]*10 + bank[top_2[1]]) as u32);

        total_joltage += (bank[top_2[0]]*10 + bank[top_2[1]]) as u32;
    }
    println!("{}", total_joltage);
}
