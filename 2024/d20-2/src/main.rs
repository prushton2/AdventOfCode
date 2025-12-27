use colored::Colorize;
use std::{collections::{HashMap, HashSet, VecDeque}, fs};

#[derive(PartialEq)]
enum Pos {
    Wall,
    Empty,
    Start,
    End
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    // let patterns: [Vec<Pos>; 2] = [
    //     vec![Pos::Wall, Pos::Empty],
    //     vec![Pos::Wall, Pos::Wall, Pos::Empty],
    // ];
    

    let map = {
        let mut m: Vec<Vec<Pos>> = vec![];
        for line in file.split('\n') {
            let mut v: Vec<Pos> = vec![];
            for char in line.chars() {
                match char {
                    '.' => v.push(Pos::Empty),
                    '#' => v.push(Pos::Wall),
                    'S' => v.push(Pos::Start),
                    'E' => v.push(Pos::End),
                    _ => {}
                }
            }
            m.push(v);
        }
        m
    };

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                Pos::Start => {start = (x, y);},
                Pos::End => {end = (x, y);},
                _ => {}
            }
        }
    }

    let (path, parents) = dijkstra(&map, start, end);
    // let mut total_faster_paths = 0;
    let mut all_valid_cheats: HashSet<((usize, usize), (usize, usize))> = [].into();

    for distance_from_start in 0..path.len() {
        let pos = path[distance_from_start];
        // println!("Checking {:?}", pos);
        let valid_cheats = find_cheats(pos, &map, &parents);

        for i in valid_cheats {
            all_valid_cheats.insert((pos, i));
        }
    }
    
    // for y in 0..map.len() {
    //     for x in 0..map[0].len() {
    //         if path.contains(&(x, y)) && (x, y) != start && (x, y) != end {
    //                 print!("{:0>7}|", format!("{}", parents.get(&(x, y)).unwrap().1).purple());
    //                 continue
    //             }
    //         match map[y][x] {
    //             Pos::Start => print!("{:0>7}|", format!("{}", parents.get(&(x, y)).unwrap().1).red()),
    //             Pos::End => print!("{:0>7}|", format!("{}",parents.get(&(x, y)).unwrap().1).green()),
    //             Pos::Wall => print!("########"),
    //             // Pos::Empty => print!("{}", ".".bright_black())
    //             Pos::Empty => print!("{:0>7}|", format!("{}", parents.get(&(x, y)).unwrap().1).cyan())
    //         }
    //     }
    //     println!("");
    // }

    // for path in &all_valid_cheats {
    //     println!("{:?} -> {:?}", path.0, path.1);
    // }

    println!("\nfaster paths: {}", all_valid_cheats.len());
    println!("path length:  {}", path.len());
}

fn find_cheats(pos: (usize, usize), map: &Vec<Vec<Pos>>, parents: &HashMap<(usize, usize), ((usize, usize), u64)>) -> HashSet<(usize, usize)> {

    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue: VecDeque<((usize, usize), u64)> = [(pos, 0)].into();
    let mut visited: HashSet<(usize, usize)> = [pos].into(); // tiles we've visited
    let mut cheats: HashSet<(usize, usize)> = [].into();// the tiles that valid cheats end on

    while queue.len() > 0 {
        let front = match queue.pop_front() {
            Some(t) => t,
            None => break
        };

        if front.1 >= 20 {
            continue;
        }

        for direction in directions {
            let target = match get_target(front.0, direction, map) {
                Some(t) => t,
                None => continue
            };

            if visited.contains(&target) {
                continue;
            }
            visited.insert(target);
            queue.push_back((target, front.1 + 1));

            if map[target.1][target.0] == Pos::Wall {
                continue;
            }
            
            // its empty, so check if its a valid cheat spot
            let target_cost = parents.get(&target).unwrap().1;
            let cost = parents.get(&pos).unwrap().1;
            let distance = front.1 + 1;
            
            if target_cost + distance < cost {
                let savings = cost - (target_cost + distance);
                if savings >= 100 {
                    cheats.insert(target);
                    // println!("Valid route from {:?} ({}) to {:?} ({}) travelling {} extra tiles saving {}", pos, cost, target, target_cost, distance, savings);
                }
            }
        }

    }

    return cheats;
}

fn get_target(pos: (usize, usize), offset: (isize, isize), map: &Vec<Vec<Pos>>) -> Option<(usize, usize)> {
    let target = (
        (offset.0) + pos.0 as isize,
        (offset.1) + pos.1 as isize,
    );

    if target.0 < 0 || target.0 >= map[0].len() as isize || target.1 < 0 || target.1 >= map.len() as isize {
        return None;
    }

    let target = (
        target.0 as usize,
        target.1 as usize
    );

    return Some(target);
}

fn dijkstra(map: &Vec<Vec<Pos>>, start: (usize, usize), end: (usize, usize)) -> (Vec<(usize, usize)>, HashMap<(usize, usize), ((usize, usize), u64)>) {
    let mut queue: VecDeque<((usize, usize), u64)> = [(end, 0)].into(); // pathfind from end to start so that parents contains each tiles shortest distance to the end
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut parents: HashMap<(usize, usize), ((usize, usize), u64)> = [(end, (end, 0))].into();
    //                         tile             |               cost to get there
    //                                        tile that got there

    while queue.len() > 0 {
        let front = match queue.pop_front() {
            Some(t) => t,
            None => break
        };

        for direction in directions {
            let target = (
                direction.0 + front.0.0 as isize,
                direction.1 + front.0.1 as isize
            );

            if target.0 < 0 || target.0 >= map[0].len() as isize || target.1 < 0 || target.1 >= map.len() as isize {
                continue;
            }

            let target = (
                target.0 as usize,
                target.1 as usize
            );

            if map[target.1][target.0] == Pos::Wall {
                continue;
            }

            match parents.get(&target) {
                Some(t) => {
                    if t.1 > front.1 + 1 {
                        parents.insert(target, (front.0, front.1 + 1));
                        queue.push_back((target, front.1+1));
                    }
                }
                None => {
                    parents.insert(target, (front.0, front.1 + 1));
                    queue.push_back((target, front.1+1));
                }
            }
        }
    }

    let mut vec: Vec<(usize, usize)> = vec![start];
    let mut pos = start;
    while pos != end {
        pos = parents.get(&pos).unwrap().0;
        vec.push(pos);
    }

    return (vec, parents);
}

    //     // let distance_from_end = path.len() - distance_from_start - 1;

    //     for direction in directions {
    //         let mut d = 0;
    //         if can_skip(2, pos, direction, &map, &parents) {
    //             d = 2;
    //         }
    //         if can_skip(3, pos, direction, &map, &parents) {
    //             d = 3;
    //         }

    //         if d != 0 {
    //             let target = match get_target(pos, (direction.0 * d as isize, direction.1 * d as isize), &map) {
    //                 Some(t) => t,
    //                 None => continue
    //             };

    //             let target_cost = parents.get(&target).unwrap().1;
    //             let cost: u64 = parents.get(&pos).unwrap().1;

    //             let savings = cost - target_cost - d;

    //             if savings < 100 {
    //                 continue
    //             }

    //             println!("Cheat from {:?} ({}) to {:?} ({}) saving {} picoseconds", pos, cost, target, target_cost, savings);
    //             total_faster_paths += 1;
    //         }
    //     }