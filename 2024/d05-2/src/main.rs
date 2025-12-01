use std::{collections::HashMap, fs, os::unix::process};

#[derive(PartialEq, Copy, Clone)]
enum PageStatus {
    Ordered,
    Unordered,
    Unrequired,
}
fn main() {
    let binding = fs::read_to_string("./src/input.txt").expect("Error reading file");
    let file: Vec<&str> = binding.split("\n\n").collect();

    let mut dependencies: HashMap<i32, Vec<i32>> = [].into();
    let mut middle_sum = 0;

    for rule in file[0].split('\n') {
        let parts: Vec<&str> = rule.split('|').collect();
        let num = parts[1].parse::<i32>().unwrap();
        let dependency = parts[0].parse::<i32>().unwrap();

        match dependencies.get_mut(&num) {
            Some(n) => {
                n.push(dependency);
            }
            None => {
                dependencies.insert(num, vec![dependency]);
            }
        }
    }

    for job in file[1].split('\n') {
        if job == "" {
            continue;
        }

        let mut print_job: Vec<i32> = vec![];
        let mut page_status: HashMap<i32, PageStatus> = [].into();
        for i in job.split(',') {
            print_job.push(i.parse::<i32>().unwrap());
            page_status.insert(i.parse::<i32>().unwrap(), PageStatus::Unordered);
        }

        // println!("{:?}", print_job);
        let mut ordered_job: Vec<i32> = vec![];
        
        if !verify_order(&print_job, &dependencies) {
            
            for page in print_job {
                order(page, &mut ordered_job, &mut page_status, &dependencies);
            }
            middle_sum += ordered_job[ordered_job.len()/2];
        }

        // print!("{}+", ordered_job[ordered_job.len()/2]);
    }

    println!("{}", middle_sum);
}

fn order(
    page: i32,
    ordered_job: &mut Vec<i32>,
    page_status: &mut HashMap<i32, PageStatus>,
    dependencies: &HashMap<i32, Vec<i32>>,
) {
    match page_status.get(&page) {
        Some(n) if *n == PageStatus::Ordered => return,
        _ => {}
    };
    let page_dependencies = match dependencies.get(&page) {
        Some(n) => n,
        None => &vec![],
    };

    for dependency in page_dependencies {
        let dependency_is_ordered = match page_status.get(dependency) {
            Some(n) => *n,
            None => PageStatus::Unrequired,
        };

        if dependency_is_ordered == PageStatus::Unordered {
            order(*dependency, ordered_job, page_status, dependencies);
        }
    }

    ordered_job.push(page);
    let _ = page_status.insert(page, PageStatus::Ordered);
}

#[derive(Clone, Copy, PartialEq)]
enum VerifierPageStatus {
    Nonexistent,
    Verified,
    Unverified
}

fn verify_order(job: &Vec<i32>, dependencies: &HashMap<i32, Vec<i32>>) -> bool {
    let mut present_numbers: Vec<VerifierPageStatus> = vec![VerifierPageStatus::Nonexistent; 100];

    for i in job {
        present_numbers[*i as usize] = VerifierPageStatus::Unverified;
    }
    
    for i in job {
        let dependency_array = dependencies.get(i).unwrap();
        present_numbers[*i as usize] = VerifierPageStatus::Verified;

        for dependency in dependency_array {
            if present_numbers[*dependency as usize] == VerifierPageStatus::Unverified {
                return false;
            }
        }
    }


    return true
}