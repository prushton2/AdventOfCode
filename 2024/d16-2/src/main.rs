use std::cmp::{Reverse, min};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::Hash;

static DEBUG_POS: (usize, usize) = (15, 7);
static DEBUG_ALL: bool = false;

#[derive(Copy, Clone, PartialEq)]
enum Pos {
    Wall,
    Air,
    Start,
    End,
    Passed
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    pub fn to_enum(i: i32) -> Direction {
        match i%4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => Direction::North
        }
    }
    pub fn to_i32(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
    pub fn difference(&self, other: &Self) -> i32 {
        let diff = (self.to_i32() - other.to_i32()).abs();
        min(diff, 4-diff)
    }
    pub fn invert(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East =>  Direction::West,
            Direction::West =>  Direction::East,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct PathfindingPosition {
    pos: (usize, usize),
    dir: Direction,
    cost: u64
}

impl Ord for PathfindingPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.cost.cmp(&other.cost);
    }
}

impl PartialEq for PathfindingPosition {
    fn eq(&self, other: &Self) -> bool {
        return self.cost == other.cost;
    }
    fn ne(&self, other: &Self) -> bool {
        return self.cost != other.cost;
    }
}

impl PartialOrd for PathfindingPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cost.cmp(&other.cost))
    }
}

impl Eq for PathfindingPosition {

}


fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    
    let mut map: Vec<Vec<Pos>> = {
        let mut v: Vec<Vec<Pos>> = vec![];
        for line in file.split('\n') {
            v.push(line.chars().map(|c| {
                match c {
                    '#' => Pos::Wall,
                    '.' => Pos::Air,
                    'S' => Pos::Start,
                    'E' => Pos::End,
                    _ => Pos::Air
                }
            }).collect())
        }
        v
    };

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                Pos::Start => {
                    start_pos = (x, y);
                },
                Pos::End => {
                    end_pos = (x, y);
                },
                _ => {}
            }
        }
    }

    let tiles = crawl_path(&map, start_pos, end_pos, Direction::East);
    
    for tile in &tiles {
        map[tile.1][tile.0] = Pos::Passed;
    }
    
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if x == DEBUG_POS.0 && y == DEBUG_POS.1 {
                print!("X");
                continue
            }
            print!("{}", match map[y][x] {
                Pos::Wall => '#',
                Pos::Air => '.',
                Pos::Start => 'S',
                Pos::End => 'E',
                Pos::Passed => 'O'
            });
        }
        println!("");
    }
    
    println!("Spaces: {}", tiles.len());

}

fn crawl_path(map: &Vec<Vec<Pos>>, start: (usize, usize), end: (usize, usize), facing: Direction) -> HashSet<(usize, usize)> {

    // the idea here is that instead of a tile being a state, the state is a tile and a direction. ie ((0, 3) North) != ((0, 3), East)
    // each state has a cost and parents that point there, and we keep track of all valid parents

    let mut queue: BinaryHeap<Reverse<PathfindingPosition>> = BinaryHeap::new();
    let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    
    let mut min_cost = u64::MAX;
    let mut end_states: Vec<((usize, usize), Direction)> = vec![]; // this is particularly useful in knowing where to start when backtracking

    let mut cost_map: HashMap<((usize, usize), Direction), u64> = [].into();
    //                             state                   cost
    let mut parents: HashMap<((usize, usize), Direction), Vec<((usize, usize), Direction)>> = [].into();
    //                             state                            parents

    queue.push(Reverse(PathfindingPosition { pos: start, dir: facing, cost: 0 }));
    cost_map.insert((start, facing), 0);

    while queue.len() > 0 {
        let front: PathfindingPosition = match queue.pop() {
            Some(Reverse(t)) => t,
            None => break
        };

        // println!("At {:?} facing {:?} with a cost of {}", front.pos, front.dir, front.cost);
        
        // if we are costing more than the cheapest way to get to where we are, skip
        if front.cost > *cost_map.get(&(front.pos, front.dir)).unwrap_or(&u64::MAX) {
            continue
        }
        
        if front.pos == end {
            // new lowest cost?
            if front.cost < min_cost {
                // there can only be one
                min_cost = front.cost;
                end_states.clear();
                end_states.push((front.pos, front.dir));
            } else if front.cost == min_cost {
                // there can be multiple
                end_states.push((front.pos, front.dir));
            }
            continue;
        }

        for i in 0..4 {
            // get our next tile
            let direction = directions[i];
            let next_pos: (usize, usize) = (
                (front.pos.0 as isize + direction.0) as usize,
                (front.pos.1 as isize + direction.1) as usize,
            );

            if map[next_pos.1][next_pos.0] != Pos::Air && map[next_pos.1][next_pos.0] != Pos::End {
                continue;
            }
            
            let next_dir = Direction::to_enum(i as i32);
            let next_cost = front.cost + 1 + (front.dir.difference(&next_dir) as u64) * 1000;

            let next_best_cost = cost_map.get(&(next_pos, next_dir)).unwrap_or(&u64::MAX);

            // are we outperforming the current cheapest way to get there?
            if next_cost < *next_best_cost {
                cost_map.insert((next_pos, next_dir), next_cost);
                parents.insert((next_pos, next_dir), vec![(front.pos, front.dir)]);
                queue.push(Reverse(PathfindingPosition{pos: next_pos, dir: next_dir, cost: next_cost}));
            } else if next_cost == *next_best_cost {
                parents.entry((next_pos, next_dir)).or_default().push((front.pos, front.dir));
                // queue.push(Reverse(PathfindingPosition{pos: next_pos, dir: next_dir, cost: next_cost}));
            }
        }
    }

    println!("Backtracking");

    // println!("{:?}\n\n{:?}\n\n", parents, cost_map);

    let mut set: HashSet<(usize, usize)> = [end_states[0].0].into();
    let mut checked_states: HashSet<((usize, usize), Direction)> = [].into();
    let mut queue: VecDeque<((usize, usize), Direction)> = end_states.into();

    while queue.len() > 0 {
        let front: ((usize, usize), Direction) = match queue.pop_front() {
            Some(t) => t,
            None => break,
        };

        // if we checked this state, skip
        if checked_states.contains(&front) {
            continue;
        }

        checked_states.insert(front);
        set.insert(front.0); // this state, since it comes from end and end contains only sthe shortest paths, has to be correct

        let parents = match parents.get(&front) { // get the parents of the state
            Some(t) => t,
            None => continue
        };

        for parent in parents { // push them to the queue
            queue.push_back(*parent);
        }

    }

    return set;
}   