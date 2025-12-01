use std::fs;
#[derive(Copy, Clone)]
enum Direction {
    Up = 1,
    Right = 2,
    Down = 4,
    Left = 8,
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");

    let directions = [(-1, 0, Direction::Left), (1, 0, Direction::Right), (0, -1, Direction::Up), (0, 1, Direction::Down)];

    let mut map: Vec<Vec<char>> = {
        let mut v: Vec<Vec<char>> = vec![];
        for line in file.split('\n') {
            if line == "" {
                continue;
            }
            v.push(line.chars().collect());
        }
        v
    };

    let mut perimeters: Vec<Vec<u8>> = vec![];

    for y in 0..map.len() as isize {
        perimeters.push(vec![]);

        for x in 0..map[y as usize].len() as isize {
            let mut fence_directions: u8 = 0;
            
            for direction in &directions {
                let target: (isize, isize) = (direction.0 + x, direction.1 + y);
                let dir_enum = direction.2 as Direction;
            
                if target.0 < 0 || target.0 >= map[y as usize].len() as isize || target.1 < 0 || target.1 >= map.len() as isize {
                    fence_directions |= dir_enum as u8;
                    continue;
                }

                if map[target.1 as usize][target.0 as usize] != map[y as usize][x as usize] {
                    fence_directions |= dir_enum as u8;
                    continue;
                }
            }

            let size = perimeters.len()-1;

            perimeters[size].push(fence_directions);
        }
    }


    for y in 0..perimeters.len() {
        for x in 0..perimeters[y as usize].len() {
            print!("{:>2} ", perimeters[y][x]);
        }
        println!("");
    }
    
    let mut total_cost = 0;

    for y in 0..map.len() {
        for x in 0..map[y as usize].len() {
            total_cost += calculate_and_pop_region(x as isize, y as isize, &mut map, &perimeters);
        }
    }

    println!("{:?}", total_cost);

}

fn calculate_and_pop_region(x: isize, y: isize, map: &mut Vec<Vec<char>>, perimeters: &Vec<Vec<u8>>) -> u32 {
    let region_indicator = map[y as usize][x as usize];
    if region_indicator == '_' {
        return 0;
    }

    let mut area = 0;
    let mut perimeter = 0;
    let mut unchecked_positions: Vec<(usize, usize)> = vec![(x as usize, y as usize)];
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    print!("Checking {},{} ({})", x, y, region_indicator);

    while unchecked_positions.len() > 0 {
        let pos: (usize, usize) = unchecked_positions.pop().unwrap();
        if map[pos.1 as usize][pos.0 as usize] == '_' {
            continue;
        }
        map[pos.1 as usize][pos.0 as usize] = '_';

        area += 1;
        perimeter += perimeters[pos.1 as usize][pos.0 as usize] as u32;
        
        for direction in directions {
            let target: (isize, isize) = (direction.0 + pos.0 as isize, direction.1 + pos.1 as isize);
            
            if target.0 < 0 || target.0 >= map[y as usize].len() as isize || target.1 < 0 || target.1 >= map.len() as isize {
                continue;
            }

            if map[target.1 as usize][target.0 as usize] == region_indicator {
                unchecked_positions.push((target.0 as usize, target.1 as usize));
                continue;
            }
        }
    }

    println!("= {} * {} = {}", area, perimeter, area*perimeter);

    return area * perimeter
}