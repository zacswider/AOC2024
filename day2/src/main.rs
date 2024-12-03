use std::{cmp::Ordering, io::BufRead, ops::Range};

#[derive(Clone, Debug)]
struct Line {
    levels: Vec<i8>,
}

impl Line {
    fn new_from_string(str: &str) -> Self {
        let words: Vec<&str> = str.split_whitespace().collect();
        let levels: Vec<i8> = words
            .iter()
            .map(|x| x.parse::<i8>().expect("Failed to parse"))
            .collect();
        Self { levels }
    }
}

fn is_safe(levels: &Vec<i8>) -> bool {
    let is_decreasing = levels[1] < levels[0];
    for idx in (Range {
        start: 1,
        end: levels.len(),
    }) {
        let prev = levels[idx - 1];
        let curr = levels[idx];
        match curr.cmp(&prev) {
            Ordering::Greater if is_decreasing => return false,
            Ordering::Less if !is_decreasing => return false,
            Ordering::Equal => return false,
            _ => {}
        }
        let diff = curr - prev;
        if diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn is_safe_with_retry(levels: &Vec<i8>) -> bool {
    if is_safe(&levels) {
        return true;
    } else {
        for idx in (Range {
            start: 0,
            end: levels.len(),
        }) {
            let mut copied = levels.clone();
            copied.remove(idx);
            let safe = is_safe(&copied);
            if safe {
                return true;
            }
        }
        return false;
    }
}

fn load_input(input_path: &str) -> Vec<Line> {
    let file = std::fs::File::open(input_path).expect("Failed to open input_path");
    let reader = std::io::BufReader::new(file);
    let mut lines: Vec<Line> = Vec::new();
    for line in reader.lines() {
        let line: Line = Line::new_from_string(&line.expect("Failed to read line"));
        lines.push(line);
    }
    lines
}

fn part1(lines: &Vec<Line>) -> () {
    let mut safe_lines = 0;
    for line in lines.iter() {
        if is_safe(&line.levels) {
            safe_lines += 1;
        }
    }
    println!("Part 1: {}", safe_lines)
}

fn part2(lines: &Vec<Line>) -> () {
    let mut safe_lines = 0;
    for line in lines.iter() {
        if is_safe_with_retry(&line.levels) {
            safe_lines += 1;
        }
    }
    println!("Part 2: {}", safe_lines)
}

fn main() {
    let start = std::time::Instant::now();
    let p = "/Users/zacswider/code/AOC2024/day2/days2.txt";
    let lines = load_input(p);
    part1(&lines);
    part2(&lines);
    let duration = start.elapsed();
    println!("time elapsed: {:?}", duration)
}
