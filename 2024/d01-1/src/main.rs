use std::fs;

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Bad file read");
    let array: Vec<&str> = file.split(&['\n', ' '][..]).collect();

    let mut lhs: Vec<i32> = vec![];
    let mut rhs: Vec<i32> = vec![];
    let mut total_diff = 0;

    let mut i = 0;
    for item in array {
        if item == "" {
            continue;
        }

        if i % 2 == 0 {
            lhs.push(item.parse::<i32>().unwrap());
        } else {
            rhs.push(item.parse::<i32>().unwrap());
        }
        i += 1;
    }

    lhs = radix_sort(&lhs);
    rhs = radix_sort(&rhs);

    for i in 0..lhs.len() {
        let mut diff = lhs[i] - rhs[i];
        if diff < 0 {
            total_diff -= diff;
        } else {
            total_diff += diff;
        }
    }

    let mut prev = 0;
    for i in &lhs {
        if *i < prev {
            println!("LHS: {} !< {}", i, prev);
        }
        prev = *i;
        // println!("LHS {}", i);
    }

    prev = 0;
    for i in &rhs {
        if *i < prev {
            println!("RHS: {} !< {}", i, prev);
        }
        prev = *i;
        // println!("RHS {}", i);
    }



    println!("{}", total_diff);
    
}

fn radix_sort(vec: &Vec<i32>) -> Vec<i32> {
    let mut max = 0;
    let mut output: Vec<i32> = vec.clone();
    for item in vec {
        if *item > max {
            max = *item;
        }
    }

    let mut place = 1;

    while place <= max {
        output = counting_sort(&output, place);
        place *= 10;
    }

    return output;
}

fn counting_sort(vec: &Vec<i32>, place: i32) -> Vec<i32> {
    let mut output: Vec<i32> = vec![0; vec.len()];
    let mut count: [i32; 10] = [0; 10];

    for i in vec {
        let digit = ((*i) / place) % 10;
        count[digit as usize] += 1;
    }

    for i in 1..10 {
        count[i] = count[i] + count[i - 1];
    }

    for i in 0..vec.len() {
        let item = vec[vec.len()-i-1];

        let digit = ((item) / place) % 10;
        count[digit as usize] -= 1;
        output[count[digit as usize] as usize] = item;
    }

    return output;
}
