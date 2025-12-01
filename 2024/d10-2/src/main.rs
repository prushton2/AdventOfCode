use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let map: Vec<Vec<u8>> = {
        let mut v: Vec<Vec<u8>> = vec![];
        for line in file.split('\n') {
            if line == "" {
                continue;
            }
            v.push(
                line.chars()
                    .map(|x| {
                        match x.to_digit(10) {
                            Some(n) => n as u8,
                            None => 15 as u8
                        }
                    })
                    .into_iter()
                    .collect(),
            );
        }
        v
    };

    let starting_locations: Vec<(usize, usize)> = {
        let mut v: Vec<(usize, usize)> = vec![];
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == 0 {
                    v.push((x, y));
                }
            }
        }
        v
    };

    let mut score_sum = 0;
    for starting_location in &starting_locations {
        score_sum += explore_point(starting_location, &map);
        // break;
    }
    println!("{:?}", score_sum);
}

fn get_valid_adjacent_locations(
    location: &(usize, usize),
    map: &Vec<Vec<u8>>,
) -> Vec<(usize, usize)> {
    let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let location_isize: (isize, isize) = (location.0 as isize, location.1 as isize);
    let value = map[location.1][location.0];

    let mut valid_targets: Vec<(usize, usize)> = vec![];
    for offset in offsets {
        let target = (location_isize.0 + offset.0, location_isize.1 + offset.1);

        if target.0 < 0
            || target.0 >= map[0].len() as isize
            || target.1 < 0
            || target.1 >= map.len() as isize
        {
            continue;
        }

        if map[target.1 as usize][target.0 as usize] != value + 1 {
            continue;
        }

        valid_targets.push((target.0 as usize, target.1 as usize));
    }

    return valid_targets;
}

fn explore_point(
    start: &(usize, usize),
    map: &Vec<Vec<u8>>
) -> i32 {
    let mut score = 0;
    for location in get_valid_adjacent_locations(start, &map) {
        // println!("Exploring {:?} ({})", location, map[location.1][location.0]);

        if map[location.1][location.0] == 9 {
            // println!("  Found a 9");
            // found_trailheads.push(location);
            score += 1;
            continue;
        }

        score += explore_point(&location, map);
    }

    return score;
}
