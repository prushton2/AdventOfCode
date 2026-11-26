use std::{collections::HashMap, fs};

struct Distance<'a> {
    pub distance: i64,
    pub a: &'a (i64, i64, i64),
    pub b: &'a (i64, i64, i64),
}

impl<'a> std::fmt::Display for Distance<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.distance).as_str())
    }
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let data: Vec<(i64, i64, i64)> = {
        let mut v = vec![];
        for line in file.split("\n") {
            if line == "" {
                continue;
            }
            let mut nums = line.split(",");
            v.push((
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap()
            ))
        }
        v
    };

    let mut distances: Vec<Distance> = vec![];
    
    for i in 0..(data.len()-1) {
        distances.extend(get_distances(&data[i..], 0));
    }

    distances.sort_by(|a, b| {
        if a.distance > b.distance {
            std::cmp::Ordering::Greater
        } else if a.distance < b.distance {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    // distances.iter().for_each(|e| println!("{}", e));

    let mut circuits: HashMap<(i64, i64, i64), usize> = [].into();

    let last_two = get_last_two(&distances, &mut circuits, data.len()).unwrap();

    println!("{} * {} = {}", last_two.0.0, last_two.1.0, last_two.0.0 * last_two.1.0);

    // let mut circuit_sizes: HashMap<usize, usize> = [].into();
    
    // for (_, value) in circuits.iter() {
    //     match circuit_sizes.get(&value) {
    //         Some(t) => { circuit_sizes.insert(*value, t + 1); },
    //         None => { circuit_sizes.insert(*value, 1); }
    //     }
    // }    
    
    // let mut largest_circuits: Vec<usize> = circuit_sizes.iter()
    //     .map(|(_, value)| *value)
    //     .collect();

    // largest_circuits.sort();
    // largest_circuits.reverse();

    // for ((x, y, z), value) in circuits.iter() {
    //     println!("({} {} {}) in {}", x, y, z, value);
    // }

    // println!("  -----  ");

    // for (id, value) in circuit_sizes.iter() {
    //     println!("Circuit {} with size {}", id, value);
    // }

    // println!("  -----  ");
    
    // println!("{} * {} * {} = {}", largest_circuits[0], largest_circuits[1], largest_circuits[2], largest_circuits[0] * largest_circuits[1] * largest_circuits[2]);

    
}

fn get_distances<'a>(data: &'a [(i64, i64, i64)], entry_index: usize) -> Vec<Distance<'a>> {
    let entry = &data[entry_index];
    
    let distances: Vec<Distance<'a>> = data
        .iter()
        .map(|e| {
            let temp = (entry.0 - e.0, entry.1 - e.1, entry.2 - e.2);
            let temp = (temp.0 * temp.0, temp.1 * temp.1, temp.2 * temp.2);

            Distance {
                distance: temp.0 + temp.1 + temp.2,
                a: entry,
                b: e
            }
        })
        .filter(|e| e.distance != 0)
        .collect();
    distances
}


fn get_last_two<'a>(distances: &'a Vec<Distance<'a>>, circuits: &mut HashMap<(i64, i64, i64), usize>, circuit_box_count: usize) -> Option<((i64, i64, i64), (i64, i64, i64))> {
    for i in 0..distances.len() {
        let distance = &distances[i];
        
        match (circuits.get(distance.a), circuits.get(distance.b)) {
            (None, None) => {
                circuits.insert(distance.a.clone(), i);
                circuits.insert(distance.b.clone(), i);
            }
            (Some(t), None) => {
                circuits.insert(distance.b.clone(), *t);
            }
            (None, Some(t)) => {
                circuits.insert(distance.a.clone(), *t);
            }
            (Some(a), Some(b)) if a == b => {
                // they are already on the same circuit, so nothing happens
            }
            (Some(a), Some(b)) if a != b => {
                // this is rough, we have to reassign all of b to a
                let a = *a;
                let b = *b;

                for (_, value) in circuits.iter_mut() {
                    if *value == b {
                        *value = a;
                    }
                }
            }
            _ => {}
        }

        // check if all boxes are on one circuit
        if circuits.len() == circuit_box_count {
            let mut flag = true;
            let mut cmp: Option<usize> = None;
            for (_, value) in circuits.iter() {
                if let Some(cmp) = cmp &&
                    cmp != *value {

                    flag = false;
                    break; 
                }

                if cmp.is_none() {
                    cmp = Some(*value);
                }
            }

            if flag {
                // println!("{} and {} are the last two to connect", distances[i], distances[i-1]);
                return Some((distances[i].a.clone(), distances[i].b.clone()));
            }
        }

    }

    return None;
}