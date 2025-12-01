use std::{collections::HashMap, fs};

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
        for i in job.split(',') {
            print_job.push(i.parse::<i32>().unwrap());
        }

        if verify_order(&print_job, &dependencies) {
            middle_sum += print_job[print_job.len()/2];
        };
    }

    println!("{}", middle_sum);
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