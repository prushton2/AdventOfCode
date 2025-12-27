use std::{collections::{HashMap, HashSet, VecDeque}, fs};
#[derive(Clone, Copy, PartialEq)]
enum Pos {
    Corrupted,
    Clear
}

static SIZE: (usize, usize) = (71, 71);
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut map: Vec<Vec<Pos>> = vec![vec![Pos::Clear; SIZE.0]; SIZE.1];
    let mut fall_order: VecDeque<(usize, usize)> = [].into();
    
    for line in file.split('\n') {
        let mut iter = line.split(',').map(|c| c.parse::<usize>().unwrap());
        fall_order.push_back((iter.next().unwrap(), iter.next().unwrap()));
    }

    sim(&mut map, 1024, &mut fall_order);

    for y in 0..SIZE.1 {
        for x in 0..SIZE.0 {
            match map[y][x] {
                Pos::Corrupted => print!("#"),
                Pos::Clear => print!(".")
            }
        }
        println!("");
    }

    println!("{:?}", dijkstra(&map, (0,0), (70,70)));
}

fn sim(map: &mut Vec<Vec<Pos>>, time: i32, fall_order: &mut VecDeque<(usize, usize)>) {
    for _ in 0..time {
        let byte = match fall_order.pop_front() {
            Some(t) => t,
            None => return
        };

        map[byte.1][byte.0] = Pos::Corrupted;
    }
}

fn dijkstra(map: &Vec<Vec<Pos>>, start: (usize, usize), end: (usize, usize)) -> Option<i32> {

    let mut queue: VecDeque<((usize, usize), i32)> = [(start, 0)].into();
    let mut visited: HashSet<(usize, usize)> = [start].into();
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while queue.len() != 0 {
        let front = match queue.pop_front() {
            Some(t) => t,
            None => break,
        };

        if front.0 == end {
            return Some(front.1)
        }

        for dir in directions {
            let target = (
                (front.0.0 as isize + dir.0),
                (front.0.1 as isize + dir.1)
            );

            if target.0 < 0 || target.0 >= SIZE.0 as isize || target.1 < 0 || target.1 >= SIZE.1 as isize{
                continue;
            }

            let target = (
                target.0 as usize,
                target.1 as usize
            );

            if map[target.1][target.0] == Pos::Corrupted {
                continue
            }

            if visited.contains(&target) {
                continue;
            }

            visited.insert(target);

            queue.push_back((target, front.1+1));
        }
    }

    return None;
}