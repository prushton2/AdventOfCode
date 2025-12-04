use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut map: Vec<Vec<char>> = {
        let mut v: Vec<Vec<char>> = vec![];
        for line in file.split('\n') {
            v.push(line.chars().collect());
        }
        v
    };

    let mut roll_count = 0;
    loop {
        let mut roll_queue: Vec<(usize, usize)> = vec![];
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != '@' {
                    continue;
                }
                
                if roll_is_accessible(&map, x, y) {
                    roll_count += 1;
                    roll_queue.push((x,y))
                };
            }
        }

        if roll_queue.len() == 0 {
            break
        }

        for roll in roll_queue {
            map[roll.1][roll.0] = '.'
        }
    }

    println!("{roll_count}");
}

fn roll_is_accessible(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let pos = (x as isize, y as isize);
    let directions: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    let mut rolls = 0;

    for dir in directions {
        let target = (pos.0 + dir.0, pos.1 + dir.1);

        if target.0 < 0 || target.1 < 0 || target.0 >= map[0].len() as isize || target.1 >= map.len() as isize {
            continue;
        }

        let target = (target.0 as usize, target.1 as usize);

        if map[target.1][target.0] == '@' {
            rolls += 1;
        }
    }

    return rolls < 4
}