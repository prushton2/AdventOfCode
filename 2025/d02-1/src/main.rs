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
        print!("{}-{}:  ", range.min, range.max - 1);
        for i in range.min..range.max {
            if id_is_invalid(i) {
                // print!("{} ", i);
                sum += i;
            }
        }
        println!("");
    }

    println!("Sum: {}", sum)
}

fn id_is_invalid(id: u64) -> bool {
    // print!("{}: ", id);
    let digits = id.ilog10() + 1;
    // if it has an odd number of digits, it doesnt have a pattern
    if digits%2 == 1 {
        return false
    }

    for i in 0..digits/2 {
        if ((id / (10 as u64).pow(i+digits/2)) % 10) != ((id / (10 as u64).pow(i)) % 10) {
            return false
        }
    }

    // let str = 

    return true;
}