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

    let mut total_joltage: i64 = 0;

    for bank in batteries {
        let mut top_12: [isize; 12] = [-1; 12];

        println!("Bank {:?}", bank);
        
        for battery_no in 0..12 {
            
            let previous_battery_index = if battery_no == 0 { 0 } else { 1+top_12[battery_no-1] as usize }; //our starting index of the range, since batteries must be in order
            let reserved_spaces = 11 - battery_no as usize; // how many spaces we need to guarantee a valid battery
            print!("\n  Battery {battery_no}\n  PBN: {}, RS: {}\n    ", previous_battery_index, reserved_spaces);
            
            for i in previous_battery_index..bank.len()-reserved_spaces  {
                print!("{} ", bank[i]);
                let battery = bank[i];

                if top_12[battery_no] == -1 || battery > bank[top_12[battery_no] as usize] {
                    top_12[battery_no] = i as isize;
                }
            }
            println!("\n    Largest: {}", bank[top_12[battery_no] as usize]);
        }

        print!("Top 12: ");
        let mut j: i64 = 0;
        for i in 0..12 {
            print!(" {}", bank[top_12[i] as usize]);
        }

        for i in 0..12 {
            j *= 10;
            j += bank[top_12[i] as usize] as i64;
        }
        println!("\nJoltage: {j}");
        total_joltage += j;
        println!("\n");
    }
    println!("{}", total_joltage);
}
