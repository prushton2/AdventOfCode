use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");

    let fresh_ids: Vec<(i64, i64)> = {
        let mut v: Vec<(i64, i64)> = vec![];
        for line in file.split("\n\n").next().unwrap().split('\n') {
            let range_nos = line.split('-').map(|e| e.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            v.push((range_nos[0], range_nos[1]));
        }
        v
    };


    let fresh_ids = radix_sort(&fresh_ids);

    let mut merged_ranges: Vec<(i64, i64)> = vec![];

    for fresh_range in fresh_ids.into_iter() {
        let mut inserted = false;
        for i in 0..merged_ranges.len() {
            let merged_range = merged_ranges[i];
            if fresh_range.0 >= merged_range.0 && fresh_range.0 <= merged_range.1 {
                if fresh_range.1 > merged_range.1 {
                    merged_ranges[i].1 = fresh_range.1;
                }
                inserted = true;
                break;
            }
        }
        if !inserted {
            merged_ranges.push(fresh_range);
        }
    }


    let mut fresh_count = 0;

    for range in &merged_ranges {
        println!("{}-{}", range.0, range.1);
        fresh_count += range.1 - range.0 + 1;
    }

    println!("{}", fresh_count);

}


fn radix_sort(vec: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut max = 0;
    let mut output: Vec<(i64, i64)> = vec.clone();
    for (item, _) in vec {
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

fn counting_sort(vec: &Vec<(i64, i64)>, place: i64) -> Vec<(i64, i64)> {
    let mut output: Vec<(i64, i64)> = vec![(0, 0); vec.len()];
    let mut count: [i64; 10] = [0; 10];

    for (i, _) in vec {
        let digit = ((*i) / place) % 10;
        count[digit as usize] += 1;
    }

    for i in 1..10 {
        count[i] = count[i] + count[i - 1];
    }

    for i in 0..vec.len() {
        let item = vec[vec.len()-i-1];

        let digit = ((item.0) / place) % 10;
        count[digit as usize] -= 1;
        output[count[digit as usize] as usize] = item;
    }

    return output;
}
