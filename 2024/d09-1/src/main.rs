use std::fs;
fn main() {
    let file: Vec<char> = fs::read_to_string("./src/input.txt")
        .expect("Error reading input")
        .chars()
        .collect();
    let mut disk: Vec<i64> = vec![]; // -1 signifies an empty block, -2 signifies a deallocated block (we cant write here, it was previously a value)
    let mut first_empty_sector: u64 = 0;

    for i in 0..file.len() / 2 + 1 {
        let file_size: u64 = file[i * 2].to_digit(10).unwrap() as u64;
        let space_size: u64 = if i * 2 + 1 < file.len() {
            file[i * 2 + 1].to_digit(10).unwrap() as u64
        } else {
            0
        };

        for _ in 0..file_size {
            disk.push(i as i64)
        }

        if first_empty_sector == 0 {
            first_empty_sector = file_size;
        }

        for _ in 0..space_size {
            disk.push(-1);
        }
    }

    for i in &disk {
        if *i == -1 {
            print!(".");
        } else {
            print!("{i}");
        }
    }
    println!("");

    for reverse_i in 0..disk.len() {
        let i = disk.len() - reverse_i-1;

        while first_empty_sector < disk.len() as u64 && disk[first_empty_sector as usize] != -1 {
            first_empty_sector += 1;
        }

        if first_empty_sector >= disk.len() as u64 {
            break;
        }

        disk[first_empty_sector as usize] = disk[i];
        disk[i] = -2;
    }

    for i in &disk {
        if *i == -1 {
            print!(".");
        } else if *i == -2 {
            print!("-");
        } else {
            print!("{i}");
        }
    }

    let mut checksum = 0;
    for i in 0..disk.len() {
        let e = disk[i];
        if e == -2 {
            continue;
        }
        checksum += i as i64 * e as i64;
    }
    println!("\n{}", checksum);
}
