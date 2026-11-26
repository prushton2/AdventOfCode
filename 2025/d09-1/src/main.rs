use std::fs;

struct Area<'a> {
    pub area: i64,
    pub a: &'a (i64, i64),
    pub b: &'a (i64, i64),
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let coordinates = file.lines()
        .map(|line| {
            let mut segments = line.split(',');
            let x = segments.next().unwrap().parse::<i64>().unwrap();
            let y = segments.next().unwrap().parse::<i64>().unwrap();

            (x, y)
        })
        .collect::<Vec<(i64, i64)>>();

        let mut distances: Vec<Area> = vec![];
    
    for i in 0..(coordinates.len()-1) {
        distances.extend(get_areas(&coordinates[i..], 0));
    }

    distances.sort_by(|a, b| {
        if a.area > b.area {
            std::cmp::Ordering::Greater
        } else if a.area < b.area {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });

    for d in distances {
        println!("({} {}) and ({} {}) has {}", d.a.0, d.a.1, d.b.0, d.b.1, d.area);
    }
    
}


fn get_areas<'a>(data: &'a [(i64, i64)], entry_index: usize) -> Vec<Area<'a>> {
    let entry = &data[entry_index];
    
    let distances: Vec<Area<'a>> = data
        .iter()
        .map(|e| {

            Area {
                area: ((entry.0 - e.0).abs() + 1) * ((entry.1 - e.1).abs() + 1),
                a: entry,
                b: e
            }
        })
        .filter(|e| e.area != 0)
        .collect();
    distances
}