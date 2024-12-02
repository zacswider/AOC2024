use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::zip;
use std::time::Instant;

fn load_input(input_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = fs::File::open(input_path).expect("failed to open");
    let reader = io::BufReader::new(file);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let words: Vec<&str> = line.split_whitespace().collect();
        let nums: Vec<i32> = words
            .iter()
            .map(|x| x.parse::<i32>().expect("Failed to parse"))
            .collect();
        assert!(nums.len() == 2);
        left.push(nums[0]);
        right.push(nums[1]);
    }
    assert!(left.len() == right.len());
    (left, right)
}

fn part1(mut left: Vec<i32>, mut right: Vec<i32>) -> () {
    left.sort();
    right.sort();

    let mut diffs: Vec<i32> = Vec::with_capacity(left.len());

    for (idx, (left_val, right_val)) in zip(left, right).enumerate() {
        let diff: i32 = right_val - left_val;
        let diff = diff.abs();
        diffs.insert(idx, diff);
    }
    let sum: i32 = diffs.iter().sum();
    println!("part 1: {}", sum);
}

fn part2(left: Vec<i32>, right: Vec<i32>) -> () {
    let mut d: HashMap<i32, i32> = HashMap::new();
    for val in right.iter() {
        let _ = d.entry(*val).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut sim_scores: Vec<i32> = Vec::with_capacity(left.len());
    for (idx, val) in left.iter().enumerate() {
        let count = d.get(val).unwrap_or(&0);
        sim_scores.insert(idx, count * val);
    }
    let sum: i32 = sim_scores.iter().sum();
    println!("part 2: {}", sum);
}

fn main() {
    let start = Instant::now();
    let p = "/Users/zacswider/code/AOC2024/day1/part1.txt";
    let (left, right) = load_input(p);
    part1(left.clone(), right.clone());
    part2(left, right);
    let duration = start.elapsed();
    println!("time elapsed: {:?}", duration)
}
