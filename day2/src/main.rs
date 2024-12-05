use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

fn read_input() -> Result<Vec<Vec<i32>>, io::Error> {
    // Specify the file path
    let path = "/home/coder/development/advent-of-code/day2/src/input.txt";

    // Open the file
    let file = File::open(path)?;

    // Use a buffered reader for efficiency
    let reader = io::BufReader::new(file);

    // Read and parse the file into a 2D vector
    let array: Vec<Vec<i32>> = reader
        .lines() // Read the file line by line
        .map(|line| {
            line.unwrap() // Unwrap the Result<String>
                .split_whitespace() // Split the line into words
                .map(|num| num.parse::<i32>().unwrap()) // Parse each word to i32
                .collect() // Collect into a Vec<i32>
        })
        .collect(); // Collect all rows into a Vec<Vec<i32>>

    Ok(array)
}

fn part1() {
    let input = read_input().unwrap();
    let mut safe_count = 0;

    for status in input {
        let mut is_safe = true;
        let mut is_increasing: Option<bool> = None;
        let mut last_level: Option<i32> = None;
        for level in status.clone() {
            match last_level {
                Some(last) => {
                    match is_increasing {
                        Some(increasing) => {
                            if increasing {
                                if last > level {
                                    is_safe = false;
                                    break;
                                }
                            } else {
                                if last < level {
                                    is_safe = false;
                                    break;
                                }
                            }
                        }
                        None => {
                            if last > level {
                                is_increasing = Some(false);
                            } else {
                                is_increasing = Some(true);
                            }
                        }
                    }

                    if last == level || last > level + 3 || last < level - 3 {
                        is_safe = false;
                        break;
                    }
                    last_level = Some(level);
                }
                None => {
                    last_level = Some(level);
                }
            }
        }
        if is_safe {
            safe_count += 1;
        }
    }

    println!("Safe Count: {}", safe_count);
}

#[derive(Copy, Clone)]
struct StatusResultError {
    last_index: Option<i32>,
    current_index: i32,
}

#[derive(Clone)]
struct StatusResult {
    status: Vec<i32>,
    error: Option<StatusResultError>,
}

fn part2() {
    let input = read_input().unwrap();
    let time = std::time::Instant::now();

    let mut results = vec![];

    for status in input {
        results.push(validate_status(status, 0));
    }

    let mut res = results.iter().filter(|r| r.error.is_none()).count();

    for result in results.clone().iter().filter(|r| r.error.is_some()) {
        let mut try1 = result.status.clone();
        try1.remove(result.error.unwrap().current_index as usize);
        let result_try1 = validate_status(try1, 1);
        
        if result_try1.error.is_none() {
            res += 1;
            continue;
        }

        if result.error.unwrap().last_index.is_none() {
            continue;
        }

        let mut try2 = result.status.clone();
        try2.remove(result.error.unwrap().last_index.expect("previous is none check failed which is impossible") as usize);
        let result_try2 = validate_status(try2, 1);

        if result_try2.error.is_none() {
            res += 1;
            continue;
        }

        if result.error.unwrap().current_index + 1 < result.status.len() as i32 {
            let mut try3 = result.status.clone();
            try3.remove((result.error.unwrap().current_index + 1) as usize);
            let result_try3 = validate_status(try3, 1);
    
            if result_try3.error.is_none() {
                res += 1;
                continue;
            }
        }

        if result.error.unwrap().last_index.unwrap() - 1 >= 0 {
            let mut try4 = result.status.clone();
            try4.remove((result.error.unwrap().last_index.unwrap() - 1) as usize);
            let result_try4 = validate_status(try4, 1);
    
            if result_try4.error.is_none() {
                res += 1;
                continue;
            }
        }
    }

    println!("Part 2 timing: {:?}", time.elapsed());
    println!("part2: Safe Count: {}", res);
}

fn validate_status(status: Vec<i32>, recursion_level: i8) -> StatusResult {
    let mut result: StatusResult = StatusResult {
        status: status.clone(),
        error: None,
    };
    if recursion_level > 1 {
        panic!("Recursion level too deep");
    }
    let mut is_increasing: Option<bool> = None;
    let mut last_level: Option<i32> = None;
    let mut last_level_index: Option<i32> = None;
    for i in 0..status.clone().len() {
        let level = status[i];
        match last_level {
            Some(last) => {
                match is_increasing {
                    Some(increasing) => {
                        if increasing {
                            if last > level {
                                result.error = Some(StatusResultError {
                                    last_index: last_level_index,
                                    current_index: i as i32,
                                });
                                break;
                            }
                        } else {
                            if last < level {
                                result.error = Some(StatusResultError {
                                    last_index: last_level_index,
                                    current_index: i as i32,
                                });
                                break;
                            }
                        }
                    }
                    None => {
                        if last > level {
                            is_increasing = Some(false);
                        } else {
                            is_increasing = Some(true);
                        }
                    }
                }

                if last == level || last > level + 3 || last < level - 3 {
                    result.error = Some(StatusResultError {
                        last_index: last_level_index,
                        current_index: i as i32,
                    });
                    break;
                }
                last_level = Some(level);
                last_level_index = Some(i as i32);
            }
            None => {
                last_level = Some(level);
                last_level_index = Some(i as i32);
            }
        }
    }
    result
}

fn main() {
    part1();
    // let time = std::time::Instant::now();
    part2();
    // println!("Timing: {:?}", time.elapsed());
}
