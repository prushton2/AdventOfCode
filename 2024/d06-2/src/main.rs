use std::fs;

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let mut map: Vec<String> = vec![];
    let mut guard_path: Vec<String>;
    let mut num_loops = 0;

    for s in file.split('\n') {
        map.push(String::from(s));
    }

    guard_path = map.clone();

    let mut guard_pos: (usize, usize) = (0, 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y].chars().nth(x).unwrap() == '^' {
                guard_pos = (x, y);
                break;
            }
        }

        if guard_pos != (0, 0) {
            break;
        }
    }

    let guard_starting_pos = guard_pos.clone();

    while move_guard(&mut guard_pos, &mut guard_path) {}

    // println!("Guard's Path");
    // for row in &guard_path {
    //     println!("{}", row);
    // }

    for y in 0..guard_path.len() {
        for x in 0..guard_path[y].len() {
            if guard_path[y].chars().nth(x).unwrap() != 'X' {
                continue;
            }

            // Setup map and guard
            map[y].replace_range(x..x + 1, "#");

            let mut guard_a_pos = (guard_starting_pos.clone(), 0 as usize);
            let mut guard_b_pos = (guard_starting_pos.clone(), 0 as usize);

            let mut loops = false;

            // Check if path gets guard stuck in a loop
            loop {
                if sim_move_guard(&mut guard_a_pos, &map) == false
                    || sim_move_guard(&mut guard_a_pos, &map) == false
                {
                    break;
                }

                sim_move_guard(&mut guard_b_pos, &map);

                if guard_a_pos.1 == guard_b_pos.1
                    && guard_a_pos.0.0 == guard_b_pos.0.0
                    && guard_a_pos.0.1 == guard_b_pos.0.1
                {
                    loops = true;
                    break;
                }
            }
            // Cleanup
            map[y].replace_range(x..x + 1, ".");

            if loops {
                num_loops += 1;
            }

            // Print results
            // println!("\nTest ({}, {}):\nLoops: {}", x, y, loops);
            // for y_2 in 0..map.len() {
            //     for x_2 in 0..map[y_2].len() {
            //         if x == x_2 && y == y_2 {
            //             print!("X")
            //         } else {
            //             print!("{}", map[y_2].chars().nth(x_2).unwrap());
            //         }
            //     }
            //     println!("")
            // }
        }
    }
    println!("{}", num_loops);
}

fn turn_right(d: char) -> char {
    let directions = ['^', '>', 'V', '<', '^'];
    for i in 0..directions.len() {
        if directions[i] == d {
            return directions[i + 1];
        }
    }

    return '^';
}

fn sim_move_guard(guard_pos: &mut ((usize, usize), usize), map: &Vec<String>) -> bool {
    let mut try_move_to = guard_pos.0.clone();

    if guard_pos.0.1 >= map.len() || guard_pos.0.0 >= map[0].len() {
        return false;
    }

    match guard_pos.1 {
        0 if try_move_to.1 != 0 => {
            try_move_to.1 -= 1;
        }
        1 => {
            try_move_to.0 += 1;
        }
        2 => {
            try_move_to.1 += 1;
        }
        3 if try_move_to.0 != 0 => {
            try_move_to.0 -= 1;
        }
        _ => {
            return false;
        }
    }

    if try_move_to.1 >= map.len() {
        return false;
    }

    let destination = match map[try_move_to.1].chars().nth(try_move_to.0) {
        Some(n) => n,
        None => {
            return false;
        }
    };

    if destination == '#' {
        guard_pos.1 += 1;
        guard_pos.1 %= 4;
    } else {
        guard_pos.0 = try_move_to.clone();
    }

    return true;
}

fn move_guard(guard_pos: &mut (usize, usize), map: &mut Vec<String>) -> bool {
    let mut try_move_to = guard_pos.clone();

    if guard_pos.1 >= map.len() || guard_pos.0 >= map[0].len() {
        map[guard_pos.1].replace_range(guard_pos.0..guard_pos.0 + 1, "X");
        return false;
    }

    let guard = map[guard_pos.1].chars().nth(guard_pos.0).unwrap();

    match guard {
        '^' => {
            if try_move_to.1 == 0 {
                map[guard_pos.1].replace_range(guard_pos.0..guard_pos.0 + 1, "X");
                return false;
            }
            try_move_to.1 -= 1;
        }
        '>' => {
            try_move_to.0 += 1;
        }
        '<' => {
            if try_move_to.0 == 0 {
                map[guard_pos.1].replace_range(guard_pos.0..guard_pos.0 + 1, "X");
                return false;
            }
            try_move_to.0 -= 1;
        }
        'V' => {
            try_move_to.1 += 1;
        }
        _ => {}
    }

    if try_move_to.1 >= map.len() {
        map[guard_pos.1].replace_range(guard_pos.0..guard_pos.0 + 1, "X");
        return false;
    }

    let destination = match map[try_move_to.1].chars().nth(try_move_to.0) {
        Some(n) => n,
        None => {
            map[guard_pos.1].replace_range(guard_pos.0..guard_pos.0 + 1, "X");
            return false;
        }
    };

    if destination == '#' {
        // turn right since theres something there
        let new_char = turn_right(guard);
        map[guard_pos.1].replace_range(guard_pos.0..guard_pos.0 + 1, new_char.to_string().as_str());

        // move_guard(guard_pos, map);
    } else {
        map[try_move_to.1]
            .replace_range(try_move_to.0..try_move_to.0 + 1, guard.to_string().as_str());

        map[guard_pos.1].replace_range(guard_pos.0..guard_pos.0 + 1, "X");

        *guard_pos = try_move_to.clone();
    }

    return true;
}
