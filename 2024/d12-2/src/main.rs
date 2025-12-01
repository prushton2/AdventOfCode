use std::{fs, u64};
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Space {
    Char(char),
    UID(u64)
}

impl Space {
    fn id(&self) -> Option<u64> {
        return match self {
            Self::Char(_) => None,
            Self::UID(c) => Some(*c)
        }
    }
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut region_info: HashMap<u64, (u64, u64)> = [].into();
    // let mut id_incrementor: u64 = 0;
    // let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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

    // Take a region and assign it a unique ID instead of a letter
    // for y in 0..map.len() as isize {
    //     for x in 0..map[y as usize].len() as isize {
    //         IDify_chunk(x, y, &mut map, &mut id_incrementor);
    //     }
    // }

    let mut total_cost = 0;

    // for y in 0..map.len() {
    //     for x in 0..map[y as usize].len() {
    //         match map[y as usize][x as usize] {
    //             Space::Char(c) => print!(" {}", c),
    //             Space::UID(c) => print!(" {:}", c)
    //         }
    //     }
    //     println!("");
    // }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let char = map[y][x];
            if char != '.' {
                let (area, walls) = calculate_chunk_corners_and_area(x as isize, y as isize, &mut map);
                println!("Region {:?} has area of {} and {} walls", char, area, walls);
                total_cost += area * walls;
            }
        }
    }

    // Solve for amount of walls in each region

    println!("{:?}", total_cost);

}

// fn calculate_chunk_area(x: isize, y: isize, map: &Vec<Vec<char>>) -> u64 {
//     let region_id = map[y as usize][x as usize];

//     let mut position_queue: Vec<(usize, usize)> = vec![(x as usize, y as usize)];
//     let mut checked_positions: Vec<(usize, usize)> = vec![];
//     let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    
//     while position_queue.len() > 0 {
//         let pos = match position_queue.pop() {
//             Some(n) => n,
//             None => continue
//         };

//         if checked_positions.contains(&pos) {
//             continue;
//         }

//         checked_positions.push(pos);

//         for direction in directions {
//             let target: (isize, isize) = (direction.0 + pos.0 as isize, direction.1 + pos.1 as isize);
            
//             if target.0 < 0 || target.0 >= map[y as usize].len() as isize || target.1 < 0 || target.1 >= map.len() as isize {
//                 continue;
//             }

//             if map[target.1 as usize][target.0 as usize] == region_id {
//                 position_queue.push((target.0 as usize, target.1 as usize));
//                 continue;
//             }
//         }
//     }
//     return checked_positions.len() as u64;
// }

fn calculate_chunk_corners_and_area(x: isize, y: isize, map: &mut Vec<Vec<char>>) -> (u64, u64) {
    /*
        All shapes we get in this puzzle have the same number of corners as sides, and this is true for most (i think) enclosed polygons.
        As such, I calculate corners by pattern matching. The pattern looks like this:

       !A  .    or    A !A
        A !A          A  A

        Either form a corner in any direction, so i test each of the 4 directions. I run this test 4 times (per each direction) on each position in the chunk.
    */

    let region_id = map[y as usize][x as usize];

    if region_id == '.' {
        return (0,0);
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
            
            if target.0 < 0 || target.0 >= map[y as usize].len() as isize || target.1 < 0 || target.1 >= map.len() as isize {
                continue;
            }

            if map[target.1 as usize][target.0 as usize] == region_id {
                position_queue.push((target.0 as usize, target.1 as usize));
                continue;
            }
        }
    }

    // now we can iterate over each square and match it to the pattern

    let patterns: [[(isize, isize); 3]; 4] = [
        [( 0, 1), ( 1, 0), ( 1, 1)], // down and right
        [( 0,-1), (-1, 0), (-1,-1)], // up and left
        [( 0,-1), ( 1, 0), ( 1,-1)], // up and right
        [( 0, 1), (-1, 0), (-1, 1)], // down and left
    ];

    let mut corners = 0;

    for position in &checked_positions {
        let c = map[position.1][position.0];

        for pattern in patterns {
            let comp_a_pos = (position.0 as isize + pattern[0].0, position.1 as isize + pattern[0].1);
            let comp_b_pos = (position.0 as isize + pattern[1].0, position.1 as isize + pattern[1].1);
            let comp_c_pos = (position.0 as isize + pattern[2].0, position.1 as isize + pattern[2].1);

            let comp_a = if comp_a_pos.0 < 0 || comp_a_pos.0 >= map[0].len() as isize || comp_a_pos.1 < 0 || comp_a_pos.1 >= map[0].len() as isize {
                '.'
            } else {
                map[comp_a_pos.1 as usize][comp_a_pos.0 as usize]
            };

            let comp_b = if comp_b_pos.0 < 0 || comp_b_pos.0 >= map[0].len() as isize || comp_b_pos.1 < 0 || comp_b_pos.1 >= map[0].len() as isize {
                '.'
            } else {
                map[comp_b_pos.1 as usize][comp_b_pos.0 as usize]
            };

            let comp_c = if comp_c_pos.0 < 0 || comp_c_pos.0 >= map[0].len() as isize || comp_c_pos.1 < 0 || comp_c_pos.1 >= map[0].len() as isize {
                '.'
            } else {
                map[comp_c_pos.1 as usize][comp_c_pos.0 as usize]
            };

            if (c == comp_a && c == comp_b && c != comp_c) 
                || (c != comp_a && c != comp_b) {
                corners += 1;
            }
        }
    }

    // Mark this as done by replacing its characters with '.' so subsequent iterations dont recalculate the region
    for i in &checked_positions {
        map[i.1][i.0] =  '.';
    }

    return (checked_positions.len() as u64, corners)
}