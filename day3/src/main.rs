use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

fn read_input() -> Result<String, io::Error> {
    // Specify the file path
    let path = "/home/coder/development/advent-of-code/day3/src/input.txt";

    // Open the file
    let file = File::open(path)?;

    // Use a buffered reader for efficiency
    let reader = io::BufReader::new(file);

    // Read and parse the file into a singular string
    Ok(reader.lines().map(|line| line.unwrap()).collect())
}

#[derive(Debug)]
enum Section {
    Do(SectionDetails),
    Dont(SectionDetails),
}

#[derive(Debug)]
struct SectionDetails {
    start: i32,
    end: Option<i32>,
}

fn calc_section(input: &str) -> Vec<i32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut results: Vec<i32> = vec![];
    // let mut results: Vec<(i32, i32)> = vec![];
    for (_, [f1, f2]) in re.captures_iter(&input).map(|caps| caps.extract()) {
        // results.push((f1.parse().unwrap(), f2.parse().unwrap()));
        results.push(f1.parse::<i32>().unwrap() * f2.parse::<i32>().unwrap());
    }

    results
}

fn part1() {
    let input = read_input().unwrap();
    // println!("{:?}", input.len());

    let re = Regex::new(r"(do|don't)\(\)").unwrap();
    let mut sections: Vec<Section> = vec![];

    sections.push(Section::Do(SectionDetails {
        start: 0,
        end: None,
    }));

    let captures = re.captures_iter(&input);
    for cap in captures {
        cap.get(0).unwrap();
        let captured_group = cap.get(0).unwrap();
        // println!(
        //     "{:?}, start at: {}, end at: {}",
        //     captured_group.as_str(),
        //     captured_group.start(),
        //     captured_group.end()
        // );

        if captured_group.as_str() == "do()" {
            sections.push(Section::Do(SectionDetails {
                start: captured_group.start() as i32,
                end: None,
            }));
            // calc_section(input.substring(captured_group.start(), captured_group.end()));
        } else {
            sections.push(Section::Dont(SectionDetails {
                start: captured_group.start() as i32,
                end: None,
            }));
        }
    }

    // println!("{:?}", sections);

    for i in 0..sections.len() {
        if i == sections.len() - 1 {
            let section = &mut sections[i];
            match section {
                Section::Do(details) | Section::Dont(details) => {
                    details.end = Some(input.len() as i32);
                }
            }
            break;
        }

        let (left, right) = sections.split_at_mut(i + 1); // Split into two mutable slices
        let sec1 = &mut left[i];
        let sec2 = &right[0];

        match sec1 {
            Section::Do(details) => match sec2 {
                Section::Dont(details2) | Section::Do(details2) => {
                    details.end = Some(details2.start);
                }
            },
            Section::Dont(details) => match sec2 {
                Section::Dont(details2) | Section::Do(details2) => {
                    details.end = Some(details2.start);
                }
            },
        }
    }

    // println!("{:?}", sections);

    let mut sum = 0;
    for section in sections {
        match section {
            Section::Do(details) => {
                let section = &input[details.start as usize..details.end.unwrap() as usize];
                // println!("{:?}", section);
                let results = calc_section(section);
                // println!("{:?}", results);
                sum += results.iter().sum::<i32>();
            }
            _ => {}
        }
    }

    // println!("amount {}", results.len());

    // let sum: i32 = results.iter().sum();

    println!("{:?}", sum);
}

fn main() {
    part1() // contains part 2 adjustments as well (oops)
}
