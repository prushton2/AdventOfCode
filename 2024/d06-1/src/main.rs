use std::fs;

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let mut map: Vec<String> = vec![];
    for s in file.split('\n') {
        map.push(String::from(s));
    }

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

    // for i in 0..10 {
    while move_guard(&mut guard_pos, &mut map) {}

    let mut unique_spaces = 0;

    for line in map {
        for char in line.chars() {
            match char {
                '^' => {unique_spaces += 1;},
                '>' => {unique_spaces += 1;},
                '<' => {unique_spaces += 1;},
                'V' => {unique_spaces += 1;},
                'X' => {unique_spaces += 1;},
                _ => {}
            }
        }
    }

    println!("{}", unique_spaces);
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

fn move_guard(guard_pos: &mut (usize, usize), map: &mut Vec<String>) -> bool {
    let mut try_move_to = guard_pos.clone();

    if guard_pos.1 >= map.len() || guard_pos.0 >= map[0].len() {
        return false;
    }

    let guard = map[guard_pos.1].chars().nth(guard_pos.0).unwrap();

    match guard {
        '^' => {
            if try_move_to.1 == 0 {
                return false;
            }
            try_move_to.1 -= 1;
        }
        '>' => {
            try_move_to.0 += 1;
        }
        '<' => {
            if try_move_to.0 == 0 {
                return false;
            }
            try_move_to.0 -= 1;
        }
        'V' => {
            try_move_to.1 += 1;
        }
        _ => {}
    }

    // println!(
    //     "Guard: {},{}\nTry: {},{}",
    //     guard_pos.0, guard_pos.1, try_move_to.0, try_move_to.1
    // );
    let destination = match map[try_move_to.1].chars().nth(try_move_to.0) {
        Some(n) => n,
        None => {
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
