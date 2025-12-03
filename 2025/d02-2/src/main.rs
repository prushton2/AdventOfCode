use std::fs;
struct Range {
    pub min: u64,
    pub max: u64
}
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let Ranges: Vec<Range> = {
        let mut v: Vec<Range> = vec![];
        for i in file.split(',') {
            let mut n = i.split('-');
            v.push(Range {
                min: n.next().unwrap().parse::<u64>().unwrap(),
                max: n.next().unwrap().parse::<u64>().unwrap()+1
            });
        }
        v
    };

    let mut sum = 0;

    for range in Ranges {
        // println!("{}-{}:  ", range.min, range.max - 1);
        for i in range.min..range.max {
            if id_is_invalid(i) {
                // print!("{} ", i);
                sum += i;
            }
        }
        // println!("");
    }

    println!("Sum: {}", sum)
}

fn id_is_invalid(id: u64) -> bool {
    // println!("  {}: ", id);
    let digits = id.ilog10() + 1;

    let mut pattern_lengths: Vec<usize> = vec![];

    for i in 1..digits/2+1 {
        if digits%i == 0 {
            pattern_lengths.push(i as usize);
        }
    }

    let str_num = format!("{}", id);
    
    for pattern_len in pattern_lengths {
        
        let mut has_match = true;
        let p = &str_num[0..pattern_len];
        // println!("    p: {p}:");
        for i in 0..str_num.len() / pattern_len {
            // println!("      {} ", &str_num[i*pattern_len..(i+1)*pattern_len]);
            if &str_num[i*pattern_len..(i+1)*pattern_len] != p {
                // println!("      false");
                has_match = false;
            }
        }

        if has_match {
            // println!("      true");
            return true
        }
    }

    return false;
}