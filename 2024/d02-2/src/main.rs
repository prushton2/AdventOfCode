#[derive(PartialEq, Debug)]
enum Direction {
    Increasing,
    Decreasing,
    Undefined,
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
        let report_status = is_report_safe(&report);
        if !report_status.0 {
            safe_reports -= 1;
        }

        match report_status.0 {
            true => print!("P "),
            false => print!("F "),
        }
        match report_status.1 != -1 {
            true => print!("D "),
            false => print!("U "),
        }

        for j in report {
            if j < 10 {
                print!(" {} ", j)
            } else {
                print!("{} ", j)
            }
        }

        print!("  ({})", report_status.1);

        println!("");
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

fn is_report_safe(report: &Vec<i32>) -> (bool, isize) {
    #[derive(PartialEq, Debug)]
    enum ReasonBroken {
        Dampened,
        Safe,
    }

    // print!("Report: ");
    // for j in report {
    //     if *j < 10 {
    //         print!(" {} ", j)
    //     } else {
    //         print!("{} ", j)
    //     }
    // }
    
    // println!("");

    let mut dampened_index: isize = -1;
    let mut is_safe = false;

    while dampened_index < (report.len() as isize) {
        let mut direction = Direction::Undefined;
        let mut prev_level = -1;
        let mut reason_broken = ReasonBroken::Safe;

        // println!("dampened_index: {}\nis_safe:{}\ndirection:{:?}\nprev_level:{}\n,reason_broken:{:?}", dampened_index, is_safe, direction, prev_level, reason_broken);

        for i in 0..(report.len() as isize) {
            // if on the index we are skipping, skip it
            if i == dampened_index {
                continue;
            }

            let level = report[i as usize];
            // define level if not defined, and skip
            if prev_level == -1 {
                prev_level = level;
                continue;
            }

            // get direction
            if direction == Direction::Undefined {
                direction = get_dir(prev_level, level);
            }

            // run checks
            if (level - prev_level).abs() > 3
                || (level - prev_level).abs() < 1
                || get_dir(prev_level, level) != direction
            {
                dampened_index += 1;
                reason_broken = ReasonBroken::Dampened;
                break;
            }

            prev_level = level;
        }

        if reason_broken == ReasonBroken::Safe {
            is_safe = true;
            break;
        }
    }

    return (is_safe, dampened_index);
}

fn get_dir(old: i32, new: i32) -> Direction {
    if old < new {
        return Direction::Increasing;
    } else {
        return Direction::Decreasing;
    }
}
