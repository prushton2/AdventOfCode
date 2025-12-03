use std::{collections::btree_set::Intersection, fs};
static GRID_SIZE: (i64, i64) = (101, 103);
// static GRID_SIZE: (i64, i64) = (11, 7);
struct Robot {
    pub pos: (i64, i64),
    pub v: (i64, i64)
}
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut robots: Vec<Robot> = {
        let mut v: Vec<Robot> = vec![];
        for line in file.split('\n') {
            let binding = line.chars().filter(|c| !(c.is_alphabetic() || c == &'=')).collect::<String>();
            // println!("{binding}");
            let mut temp = binding.split(&[',', ' ']).map(|c| c.parse::<i64>().unwrap());
            v.push(Robot { 
                pos: (
                    temp.next().unwrap(),
                    temp.next().unwrap(),
                ), 
                v: (
                    temp.next().unwrap(),
                    temp.next().unwrap(),
                ) 
            });
        }
        v
    };

    
    let mut sim_time = 0;
    loop {
        let mut grid: [[u8; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize] = [[0; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize];

        for i in 0..robots.len() {
            let robot = match robots.get_mut(i) {
                Some(n) => n,
                None => continue
            };
    
            let new_pos = (
                modulo(robot.pos.0 + robot.v.0 * sim_time, GRID_SIZE.0),
                modulo(robot.pos.1 + robot.v.1 * sim_time, GRID_SIZE.1)
            );
    
            // println!("{:?}", robot.pos);
    
            grid[new_pos.1 as usize][new_pos.0 as usize] += 1;
        }
        
        let mut interesting = false;
        
        for y in 0..GRID_SIZE.1 as usize {
            let mut s = 0;
            for x in 0..GRID_SIZE.0 as usize {
                if grid[y as usize][x as usize] > 0 {
                    s += 1;
                }
            }
            if s >= 20 {
                interesting = true;
            }
        }

        if interesting {
            println!("Sim Time: {}", sim_time);
            for y in 0..GRID_SIZE.1 as usize {
                for x in 0..GRID_SIZE.0 as usize {
                    match grid[y][x] {
                        0 => {print!(" ");}
                        _ => {print!("█");}
                    }
                }
                println!("");
            }
        }
        sim_time += 1;
    }

    // let mut bots_per_quadrant: (u64, u64, u64, u64) = (0, 0, 0, 0);

    // for robot in robots {

    //     match robot.pos {
    //         (x, y) if x > GRID_SIZE.0/2 && y < GRID_SIZE.1/2 => {bots_per_quadrant.0 += 1}
    //         (x, y) if x < GRID_SIZE.0/2 && y < GRID_SIZE.1/2 => {bots_per_quadrant.1 += 1}
    //         (x, y) if x < GRID_SIZE.0/2 && y > GRID_SIZE.1/2 => {bots_per_quadrant.2 += 1}
    //         (x, y) if x > GRID_SIZE.0/2 && y > GRID_SIZE.1/2 => {bots_per_quadrant.3 += 1}
    //         _ => {}
    //     }
    // }

    // println!("{:?}", bots_per_quadrant);
    // println!("{}", bots_per_quadrant.0 * bots_per_quadrant.1 * bots_per_quadrant.2 * bots_per_quadrant.3);

}

fn modulo(a: i64, b: i64) -> i64 {
    let mut a = a;
    while a < 0 {
        a += b;
    }
    
    a %= b;
    return a;
}

fn flood_fill(x: i64, y: i64, map: &[[u8; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize]) -> i64 {
    if map[y as usize][x as usize] == 0 {
        return 0;
    }
    
    // first thing is to get the area, and get all the positions in the region aswell.
    let mut position_queue: Vec<(usize, usize)> = vec![(x as usize, y as usize)];
    let mut checked_positions: Vec<(usize, usize)> = vec![];
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    
    while position_queue.len() > 0 {
        let pos = match position_queue.pop() {
            Some(n) => n,
            None => continue
        };

        if checked_positions.contains(&pos) {
            continue;
        }

        checked_positions.push(pos);

        for direction in directions {
            let target: (isize, isize) = (direction.0 + pos.0 as isize, direction.1 + pos.1 as isize);
            
            if target.0 < 0 || target.0 >= GRID_SIZE.0 as isize || target.1 < 0 || target.1 >= GRID_SIZE.1 as isize {
                continue;
            }

            if map[target.1 as usize][target.0 as usize] > 0 {
                position_queue.push((target.0 as usize, target.1 as usize));
                continue;
            }
        }
    }

    return checked_positions.len() as i64;
}