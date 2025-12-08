use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let rows: Vec<Vec<char>> = {
        let mut r: Vec<Vec<char>> = vec![];
        for line in file.split('\n') {
            r.push(line.chars().collect());
        }
        r
    };

    let mut S_columns: Vec<usize> = vec![];
    let mut split_count = 0;

    for row in &rows {
        for i in 0..row.len() {
            let char = row[i];
            if char == 'S' {
                S_columns = vec![i];
            }

            if char == '^' && S_columns.contains(&i) {
                let index = S_columns.iter().position(|x| *x == i).unwrap();
                S_columns.swap_remove(index);
                S_columns.extend_from_slice(&[i+1, i-1]);
                split_count += 1;
            }
        }

        S_columns = radix_sort(&S_columns);
        S_columns = dedupe_sorted(&S_columns);
    }

    println!("{}", split_count);



}

fn dedupe_sorted(vec: &Vec<usize>) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    let mut prev: usize = 0;
    
    for item in vec {
        if item != &prev {
            out.push(*item);
        }
        prev = *item;
    }

    return out

}

fn radix_sort(vec: &Vec<usize>) -> Vec<usize> {
    let mut max = 0;
    let mut output: Vec<usize> = vec.clone();
    for item in vec {
        if *item > max {
            max = *item;
        }
    }

    let mut place: i64 = 1;

    while place <= max as i64 {
        output = counting_sort(&output, place);
        place *= 10;
    }

    return output;
}

fn counting_sort(vec: &Vec<(usize)>, place: i64) -> Vec<(usize)> {
    let mut output: Vec<(usize)> = vec![0; vec.len()];
    let mut count: [i64; 10] = [0; 10];

    for i in vec {
        let digit = ((*i as i64) / place) % 10;
        count[digit as usize] += 1;
    }

    for i in 1..10 {
        count[i] = count[i] + count[i - 1];
    }

    for i in 0..vec.len() {
        let item = vec[vec.len()-i-1];

        let digit = ((item as i64) / place) % 10;
        count[digit as usize] -= 1;
        output[count[digit as usize] as usize] = item;
    }

    return output;
}
