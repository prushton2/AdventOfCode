use std::{cmp::max, collections::HashMap, fs};
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut antennae: HashMap<char, Vec<(usize, usize)>> = [].into();

    let mut map: Vec<String> = {
        let mut v: Vec<String> = vec![];
        for i in file.split('\n') {
            if i.len() == 0 {
                continue;
            }
            v.push(String::from(i))
        }
        v
    };

    for y in 0..map.len() {
        let chars: Vec<char> = map[y].chars().collect();

        for x in 0..map[y].len() {
            let char = chars[x];

            if char != '.' {
                match antennae.get_mut(&char) {
                    Some(n) => {
                        n.push((x, y));
                    }
                    None => {
                        antennae.insert(char, vec![(x, y)]);
                    }
                }
            }
        }
    }

    for element in antennae {
        for a in 0..element.1.len() {
            let antenna_a = (element.1[a].0 as isize, element.1[a].1 as isize);
            for b in 0..element.1.len() {
                if a == b {
                    continue;
                }

                let antenna_b = (element.1[b].0 as isize, element.1[b].1 as isize);

                // println!("A: {:?}\nB: {:?}", antenna_a, antenna_b);

                let mut distance = (antenna_a.0 - antenna_b.0, antenna_a.1 - antenna_b.1);
                let mut multiplier = 1;

                let half = 1 + max(distance.0.abs(), distance.1.abs()) / 2;

                for i_2 in 0..half {
                    let i = half - i_2;
                    if distance.0 % i == 0 && distance.1 % i == 0 {
                        distance = (distance.0 / i, distance.1 / i);
                    }
                }

                // println!("{}, {:?}", half, distance);

                loop {
                    let position = (
                        antenna_a.0 + distance.0 * multiplier,
                        antenna_a.1 + distance.1 * multiplier,
                    );

                    if position.0 < 0
                        || position.0 >= map[0].len() as isize
                        || position.1 < 0
                        || position.1 >= map.len() as isize
                    {
                        break;
                    }

                    let mut chars: Vec<char> = map[position.1 as usize].chars().collect();
                    chars[position.0 as usize] = '#';
                    map[position.1 as usize] = chars.into_iter().collect();
                    multiplier += 1
                }
                multiplier = -1;
                loop {
                    let position = (
                        antenna_a.0 + distance.0 * multiplier,
                        antenna_a.1 + distance.1 * multiplier,
                    );

                    if position.0 < 0
                        || position.0 >= map[0].len() as isize
                        || position.1 < 0
                        || position.1 >= map.len() as isize
                    {
                        break;
                    }

                    let mut chars: Vec<char> = map[position.1 as usize].chars().collect();
                    chars[position.0 as usize] = '#';
                    map[position.1 as usize] = chars.into_iter().collect();
                    multiplier -= 1
                }
            }
        }
    }

    let mut antinode_counter = 0;

    for row in map {
        println!("{}", row);
        for char in row.chars() {
            if char == '#' {
                antinode_counter += 1;
            }
        }
    }

    println!("{}", antinode_counter);
}
