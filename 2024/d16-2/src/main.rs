use std::cmp::min;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fs;

static DEBUG_POS: (usize, usize) = (5, 7);

#[derive(Copy, Clone, PartialEq)]
enum Pos {
    Wall,
    Air,
    Start,
    End,
    Passed
}

#[derive(Copy, Clone, PartialEq, Debug)]
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
        if diff == 3 {
            return 1
        }
        diff
    }
}

#[derive(Clone, Debug)]
struct PathfindingPosition {
    pos: (usize, usize),
    dir: Direction,
    cost: u64,
    depth: u64
    // past_tiles: HashSet<(usize, usize)>
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

fn crawl_path(map: &Vec<Vec<Pos>>, start: (usize, usize), end: (usize, usize), facing: Direction) -> Vec<(usize, usize)> {

    let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut min_cost = u64::MAX;
    let mut queue: VecDeque<PathfindingPosition> = [PathfindingPosition{pos: start, dir: facing, cost: 0, depth: 0}].into();
    let mut visited: HashMap<(usize, usize), u64> = [].into();
    let mut parents: HashMap<(usize, usize), Vec<(usize, usize)>> = [].into();

    while queue.len() > 0 {
        let front: PathfindingPosition = match queue.pop_front() {
            Some(t) => t,
            None => break
        };

        let pos = front.pos;
        // println!("\n[{}]; {:?}: ", queue.len(), pos);

        if front.cost > min_cost {
            continue
        }

        if pos == end {
            min_cost = front.cost;
            continue
        }

        for i in 0..4 {
            let direction = directions[i];
            let target: (usize, usize) = (
                (pos.0 as isize + direction.0) as usize,
                (pos.1 as isize + direction.1) as usize,
            );

            // print!("\n  {:?}: ", target);

            match visited.get(&target) {
                Some(i) => {
                    if front.depth > *i {
                        continue;
                    }
                },
                None => {
                    visited.insert(target, front.depth+1);
                }
            }
            
            if map[target.1][target.0] == Pos::Air || map[target.1][target.0] == Pos::End {

                let new_dir = Direction::to_enum(i as i32);
                let new_cost = front.cost + 1 + (front.dir.difference(&new_dir) as u64) * 1000;

                if target == DEBUG_POS {
                    println!("from {:?} ({}) facing {:?} moving {:?} cost is {}", pos, front.cost, front.dir, new_dir, new_cost);
                }

                match parents.get_mut(&target) {
                    Some(t) => {
                        t.push(pos);
                    },
                    None => {
                        parents.insert(target, vec![pos]);
                    }
                }
                
                queue.push_back(PathfindingPosition { pos: target, dir: new_dir, cost: new_cost, depth: front.depth + 1});
            }
        }
        // println!("");
        queue = radix_sort(&queue);
    }

    println!("Backtracking");

    let mut queue: Vec<(usize, usize)> = [end].into();
    let mut index = 0;

    while index < queue.len() {
        let front = queue[index];

        let parents = match parents.get(&front) {
            Some(t) => t,
            None => {index += 1; continue}
        };

        for i in parents {
            if !queue.contains(i) {
                queue.push(*i);
            }
        }
        index += 1;
    }

    return queue;
}



fn radix_sort(vec: &VecDeque<PathfindingPosition>) -> VecDeque<PathfindingPosition> {
    let mut max = 0;
    let mut output: VecDeque<PathfindingPosition> = vec.clone();
    for item in vec {
        if item.cost > max {
            max = item.cost;
        }
    }

    let mut place = 1;

    while place <= max {
        output = counting_sort(&output, place);
        place *= 10;
    }

    return output;
}

fn counting_sort(vec: &VecDeque<PathfindingPosition>, place: u64) -> VecDeque<PathfindingPosition> {
    let mut output: VecDeque<PathfindingPosition> = vec![PathfindingPosition { pos: (0, 0), dir: Direction::North, cost: 0, depth: 0}; vec.len()].into();
    let mut count: [i32; 10] = [0; 10];

    for i in vec {
        let digit = (i.cost / place) % 10;
        count[digit as usize] += 1;
    }

    for i in 1..10 {
        count[i] = count[i] + count[i - 1];
    }

    for i in 0..vec.len() {
        let item = &vec[vec.len()-i-1];

        let digit = (item.cost / place) % 10;
        count[digit as usize] -= 1;
        output[count[digit as usize] as usize] = item.clone();
    }

    return output;
}