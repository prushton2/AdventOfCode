use std::fs;

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
    Object,
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
    
    let mut map: Vec<Vec<Object>> = vec![];
    for line in binding[0].split('\n') {
        map.push(line.chars().map(|c|  {
            return match c {
                '#' => Object::Wall,
                '@' => Object::Robot,
                'O' => Object::Object,
                '.' => Object::Empty,
                d => panic!("Unexpected character {d} in map input")
            }
        }).collect());
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
        let dir = move_to_dir(instruction);
        // println!("\nDir: {:?} ", dir);
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
            if map[y][x] == Object::Object {
                gps_sum += y*100+x;
            }
        }
    }

    println!("Sum: {gps_sum}");


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
        Object::Object => {
            if move_object(map, target_pos, dir) {
                map[target_pos.1][target_pos.0] = Object::Object;
                map[pos.1][pos.0] = Object::Empty;
                return true;
            }
            return false
        }
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
                Object::Object => print!("O")
            }
        }
        println!("");
    }
}