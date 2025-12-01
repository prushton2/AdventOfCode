use std::{collections::HashMap, fs};
fn main() {
    let file: Vec<char> = fs::read_to_string("./src/input.txt")
        .expect("Error reading input")
        .chars()
        .collect();
    let mut disk: Vec<i64> = vec![]; //-1 means empty, -2 means deallocated. 

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

        for _ in 0..space_size {
            disk.push(-1);
        }
    }

    // for i in &disk {
    //     if *i == -1 {
    //         print!(".");
    //     } else {
    //         print!("{i}");
    //     }
    // }
    // println!("");
    let mut scanned_sectors: HashMap<i64, bool> = [].into(); 

    loop {
        let mut full_sector_id: i64 = -2;
        let mut full_sector_start: usize = disk.len();
        let mut full_sector_size: usize = 0;

        for i_2 in 0..disk.len() {
            let i = disk.len()-i_2-1;
            // print!("{} {}: ", i, disk[i]);
            
            // are we in the same sector as the last iteration?
            if disk[i] == full_sector_id {
                // println!("Continuing Sector");
                full_sector_start = i;
                full_sector_size += 1;
                continue;
            }

            // we entered a different sector when scanning a sector
            if disk[i] != full_sector_id && full_sector_id != -2 {
                // println!("Ended up in different sector");
                break;
            }
            
            // have we scanned this sector yet? if so, just skip it
            if scanned_sectors.get(&disk[i]) != None {
                // println!("Scanned");
                continue;
            }

            // if we encounter -1 when looking for a sector to scan, skip
            if full_sector_id == -2 && disk[i] == -1 {
                // println!("Found empty when searching for block, skipping");
                continue;
            }
            
            // define our sector
            if full_sector_id == -2 {
                // println!("New Sector");
                full_sector_id = disk[i];
                full_sector_start = i;
                full_sector_size += 1;
                continue;
            }
            
            // we are in a different sector, so break
            // println!("No conditions passed");
            break;
        }

        let _ = scanned_sectors.insert(full_sector_id, true);

        if full_sector_start == 0 {
            break;
        }

        // find an empty sector

        let mut empty_sector_start = 0;
        let mut empty_sector_size = 0;

        for i in 0..disk.len() {
            if disk[i] != -1 {
                empty_sector_start = i + 1;
                empty_sector_size = 0;
            } else {
                empty_sector_size += 1;
            }

            // we found a valid sector
            if empty_sector_size >= full_sector_size  && empty_sector_start < full_sector_start {
                for i in full_sector_start..full_sector_start + full_sector_size {
                    disk[i] = -1;
                }

                for i in empty_sector_start..empty_sector_start + empty_sector_size {
                    disk[i] = full_sector_id;
                }

                break;
            }
        }

        // for i in &disk {
        //     if *i == -1 {
        //         print!(".");
        //     } else {
        //         print!("{i}");
        //     }
        // }
        // println!("");
    }

    let mut checksum = 0;
    for i in 0..disk.len() {
        let e = disk[i];
        if e == -1 {
            continue;
        }
        checksum += i as i64 * e as i64;
    }
    println!("\n{}", checksum);
}
