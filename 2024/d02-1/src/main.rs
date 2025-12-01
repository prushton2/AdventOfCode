#[derive(PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}
use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Bad file read");
    let mut reports: Vec<Vec<i32>> = vec![vec![]];
    // let big_array: Vec<&str> = file.split('\n').collect();
    
    let mut level = 0;
    for char in file.chars() {
        let len = reports.len() - 1;
        match char {
            ' ' => {
                reports[len].push(level);
                level = 0;
            }
            '\n' => {
                reports[len].push(level);
                reports.push(vec![]);
                level = 0;
            }
            _ => {
                if char.is_numeric() {
                    level *= 10;
                    level += char.to_digit(10).unwrap() as i32;
                }
            }
        }
    }

    reports.pop();
    let mut safe_reports = reports.len();

    for report in reports {
        let mut prev_level = report[0];
        let direction = get_dir(report[0], report[1]);

        for i in 1..report.len() {
            let level = report[i];

            if (level - prev_level).abs() > 3  || (level - prev_level).abs() < 1 {
                safe_reports -= 1;
                break;
            }

            if get_dir(prev_level, level) != direction {
                safe_reports -= 1;
                break;
            }

            prev_level = level;
        }
    }

    println!("{}", safe_reports);

    // for i in reports {
    //     for j in i {
    //         if j < 10 {
    //             print!(" {} ", j)
    //         } else {
    //             print!("{} ", j)
    //         }
    //     }
    //     println!("")
    // }
}

fn get_dir(old: i32, new: i32) -> Direction {
    if old < new {
        return Direction::Increasing;
    } else {
        return Direction::Decreasing;
    }
}
