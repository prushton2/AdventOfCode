use std::{collections::{HashMap, VecDeque}, fs, u64};

#[derive(Copy, Clone, PartialEq)]
enum Pos {
    Wall,
    Air,
    Start,
    End
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
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    
    let map: Vec<Vec<Pos>> = {
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
    
    println!("Shortest path: {}", crawl_path(&map, start_pos, end_pos, Direction::East));

}

fn crawl_path(map: &Vec<Vec<Pos>>, start: (usize, usize), end: (usize, usize), facing: Direction) -> u64 {
    let directions: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut queue: VecDeque<((usize, usize), Direction, u64)> = [(start, facing, 0)].into();
    let mut checked_positions: HashMap<(usize, usize), bool> = [].into();

    while queue.len() > 0 {
        let front = match queue.pop_front() {
            Some(t) => t,
            None => break
        };
        let pos = front.0;

        if pos == end {
            return front.2;
        }

        for i in 0..4 {
            let direction = directions[i];
            let target: (usize, usize) = (
                (pos.0 as isize + direction.0) as usize,
                (pos.1 as isize + direction.1) as usize,
            );

            if (map[target.1][target.0] == Pos::Air || map[target.1][target.0] == Pos::End) && checked_positions.get(&target).is_none() {
                let new_dir = Direction::to_enum(i as i32);
                let new_cost = front.2 + 1 + if new_dir != front.1 { 1000 } else { 0 };
                checked_positions.insert(target, true);

                queue.push_back((target, new_dir, new_cost));
            }
        }

        queue = radix_sort(&queue);
    }
    return 0;
}



fn radix_sort(vec: &VecDeque<((usize, usize), Direction, u64)>) -> VecDeque<((usize, usize), Direction, u64)> {
    let mut max = 0;
    let mut output: VecDeque<((usize, usize), Direction, u64)> = vec.clone();
    for item in vec {
        if item.2 > max {
            max = item.2;
        }
    }

    let mut place = 1;

    while place <= max {
        output = counting_sort(&output, place);
        place *= 10;
    }

    return output;
}

fn counting_sort(vec: &VecDeque<((usize, usize), Direction, u64)>, place: u64) -> VecDeque<((usize, usize), Direction, u64)> {
    let mut output: VecDeque<((usize, usize), Direction, u64)> = vec![((0, 0), Direction::North, 0); vec.len()].into();
    let mut count: [i32; 10] = [0; 10];

    for i in vec {
        let digit = ((i.2) / place) % 10;
        count[digit as usize] += 1;
    }

    for i in 1..10 {
        count[i] = count[i] + count[i - 1];
    }

    for i in 0..vec.len() {
        let item = vec[vec.len()-i-1];

        let digit = ((item.2) / place) % 10;
        count[digit as usize] -= 1;
        output[count[digit as usize] as usize] = item;
    }

    return output;
}