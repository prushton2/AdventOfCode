use std::fs;

#[derive(Debug)]
enum Move {
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4
}

#[derive(Copy, Clone, PartialEq)]
enum Object {
    Empty,
    Robot,
    ObjectLeft,
    ObjectRight,
    Wall
}


fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let binding = file.split("\n\n").collect::<Vec<&str>>();
    
    let moves = binding[1].chars().filter(|c| c != &'\n').map(|c| {
        return match c {
            '^' => Move::Up,
            '<' => Move::Left,
            'v' => Move::Down,
            '>' => Move::Right,
            d => panic!("Unexpected character {d} in movement input")
        }
    });

    let mut obj_count = 0;
    
    let mut map: Vec<Vec<Object>> = vec![];
    for line in binding[0].split('\n') {
        let mut v: Vec<Object> = vec![];
        for char in line.chars() {
            let chars = match char {
                '#' => (Object::Wall, Object::Wall),
                '@' => (Object::Robot, Object::Empty),
                'O' => {obj_count += 1; (Object::ObjectLeft, Object::ObjectRight)},
                '.' => (Object::Empty, Object::Empty),
                d => panic!("Unexpected character {d} in map input")
            };
            v.push(chars.0);
            v.push(chars.1);
        }
        map.push(v);
    }

    let mut robot_pos: (usize, usize) = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Object::Robot {
                robot_pos = (x, y);
            }
        }
    }

    print_map(&map);


    for instruction in moves {
        // println!("\nDir: {:?} ", instruction);
        let dir = move_to_dir(instruction);
        if move_object(&mut map, robot_pos, dir) {
            robot_pos = (
                ((robot_pos.0 as isize) + dir.0) as usize, 
                ((robot_pos.1 as isize) + dir.1) as usize
            );
        }
    }
    
    print_map(&map);

    let mut gps_sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Object::ObjectLeft {
                println!("{x}, {y}");
                gps_sum += y*100+x;
                obj_count -= 1;
            }
        }
    }

    println!("Sum: {gps_sum}\nObj difference: {obj_count}"); // sum should be 1319212


}

fn move_to_dir(m: Move) -> (isize, isize) {
    match m {
        Move::Down => ( 0, 1),
        Move::Left => (-1, 0),
        Move::Right => (1, 0),
        Move::Up => (0, -1)
    }
}

fn move_object(map: &mut Vec<Vec<Object>>, pos: (usize, usize), dir: (isize, isize)) -> bool {

    let target_pos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);

    if target_pos.0 < 0 || target_pos.0 >= map[0].len() as isize || target_pos.1 < 0 || target_pos.1 >= map.len() as isize {
        return false; // cant move off map
    }

    let target_pos = (target_pos.0 as usize, target_pos.1 as usize);
    let pos_char = map[pos.1][pos.0];

    match pos_char {
        Object::Wall => {
            return false
        },
        Object::Robot => {
            if move_object(map, target_pos, dir) {
                map[target_pos.1][target_pos.0] = Object::Robot;
                map[pos.1][pos.0] = Object::Empty;
                return true;
            }
            return false
        },
        Object::ObjectLeft => {
            let twin_pos = (pos.0+1, pos.1);
            let twin_target = (target_pos.0+1, target_pos.1);

            if dir.1 != 0 {
                if move_object(map, target_pos, dir) && move_object(map, twin_target, dir) {
                    map[target_pos.1][target_pos.0] = Object::ObjectLeft;
                    map[twin_target.1][twin_target.0] = Object::ObjectRight;
                    map[pos.1][pos.0] = Object::Empty;
                    map[twin_pos.1][twin_pos.0] = Object::Empty;
                    return true
                }
                return false;
            }

            if move_object(map, target_pos, dir) {
                map[target_pos.1][target_pos.0] = Object::ObjectLeft;
                map[pos.1][pos.0] = Object::Empty;
                return true;
            }

            return false
        },
        Object::ObjectRight => {
            let twin = (pos.0-1, pos.1);
            if dir.1 != 0 {
                return move_object(map, twin, dir)
            }

            if move_object(map, target_pos, dir) {
                map[target_pos.1][target_pos.0] = Object::ObjectRight;
                map[pos.1][pos.0] = Object::Empty;
                return true;
            }

            return false
        },
        Object::Empty => {
            return true
        }
    }
}

fn print_map(map: &Vec<Vec<Object>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                Object::Empty => print!("."),
                Object::Robot => print!("@"),
                Object::Wall => print!("#"),
                Object::ObjectLeft => print!("["),
                Object::ObjectRight => print!("]")
            }
        }
        println!("");
    }
}