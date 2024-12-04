use std::{cmp::min, io::BufRead};

fn load_input(input_path: &str) -> Vec<Vec<char>> {
    let file = std::fs::File::open(input_path).expect("Failed to open input path");
    let reader = std::io::BufReader::new(file);
    let mut word_search: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line").chars().collect();
        word_search.push(line);
    }
    word_search
}

fn adj_indices(loc: &(usize, usize), n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
    let mut lowest_y = 0;
    if loc.0 > 0 {
        lowest_y = loc.0 - 1;
    }
    let mut lowest_x = 0;
    if loc.1 > 0 {
        lowest_x = loc.1 - 1;
    }
    let largest_y = min(loc.0 + 1, n_cols - 1);
    let largest_x = min(loc.1 + 1, n_rows - 1);
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    for y in lowest_y..largest_y + 1 {
        for x in lowest_x..largest_x + 1 {
            let neighbor = &(y, x);
            if neighbor == loc {
                continue;
            }
            neighbors.push((y, x));
        }
    }
    neighbors
}

fn next_loc(prev: &(usize, usize), curr: &(usize, usize)) -> (i16, i16) {
    let (y_prev, x_prev) = *prev;
    let (y_curr, x_curr) = *curr;
    let y_delta = y_curr as i16 - y_prev as i16;
    // println!("y delta = {:?}", y_delta);
    let x_delta = x_curr as i16 - x_prev as i16;
    // println!("x delta = {:?}", x_delta);
    (y_curr as i16 + y_delta, x_curr as i16 + x_delta)
}

fn search_along_axis(
    word_search: &Vec<Vec<char>>,
    start: &(usize, usize),
    curr: &(usize, usize),
    n_rows: usize,
    n_cols: usize,
) -> bool {
    // println!("searching along axis");
    let (next_y, next_x) = next_loc(start, curr);
    // println!("next y, x = {:?}, {:?}", next_y, next_x);
    if (next_y < 0) || (next_y > n_rows as i16 - 1) || (next_x < 0) || (next_x > n_cols as i16 - 1)
    {
        // println!("out of bounds 1!");
        return false;
    }
    let val = word_search[next_y as usize][next_x as usize];
    if val.to_string() == "A" {
        let (next_y, next_x) = next_loc(curr, &(next_y as usize, next_x as usize));
        if (next_y < 0)
            || (next_y > n_rows as i16 - 1)
            || (next_x < 0)
            || (next_x > n_cols as i16 - 1)
        {
            // println!("out of bounds 1!");
            return false;
        }
        // println!("next y, x = {:?}, {:?}", next_y, next_x);
        let val = word_search[next_y as usize][next_x as usize];
        if val.to_string() == "S" {
            return true;
        }
    }

    false
}

fn part1(word_search: &Vec<Vec<char>>) {
    let mut counts = 0;
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for (row, line) in word_search.iter().enumerate() {
        for (col, val) in line.iter().enumerate() {
            if val.to_string() == "X" {
                starts.push((row, col));
            }
        }
    }
    let n_rows = word_search.len();
    let n_cols = word_search[0].len();
    // println!("found {:?} starts", starts.len());
    // println!("nrows = {:?}", n_rows);
    // println!("ncols = {:?}", n_cols);
    for loc_tuple in starts.iter() {
        // println!("location: {:?}", &loc_tuple);
        let neighbors = adj_indices(loc_tuple, n_rows, n_cols);
        for neighbor in neighbors.iter() {
            // println!("\tneighbor: {:?}", neighbor);
            let neighbor_val = word_search[neighbor.0][neighbor.1];
            // println!("\tneighbor val: {:?}", neighbor_val);

            if neighbor_val.to_string() == "M" {
                // println!("\tcandidate!");
                if search_along_axis(&word_search, loc_tuple, neighbor, n_rows, n_cols) {
                    counts += 1;
                }
            }
        }
    }
    println!("part 1: {:?}", counts);
}

fn is_valid_tuple(t: (&str, &str, &str)) -> bool {
    t == ("M", "A", "S") || t == ("S", "A", "M")
}

fn part2(word_search: &Vec<Vec<char>>) {
    let mut counts = 0;
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for (row, line) in word_search.iter().enumerate() {
        for (col, val) in line.iter().enumerate() {
            if val.to_string() == "A" {
                starts.push((row, col));
            }
        }
    }
    let n_rows = word_search.len();
    let n_cols = word_search[0].len();
    // println!("found {:?} starts", starts.len());
    // println!("nrows = {:?}", n_rows);
    // println!("ncols = {:?}", n_cols);

    for start in starts {
        let (y_start, x_start) = start;
        if (y_start == 0) || (y_start == n_rows - 1) || (x_start == 0) || (x_start == n_cols - 1) {
            continue;
        }
        let top_left = word_search[y_start - 1][x_start - 1].to_string();
        let bottom_right = word_search[y_start + 1][x_start + 1].to_string();
        let top_right = word_search[y_start - 1][x_start + 1].to_string();
        let bottom_left = word_search[y_start + 1][x_start - 1].to_string();

        let left = (top_left.as_str(), "A", bottom_right.as_str());

        let right = (top_right.as_str(), "A", bottom_left.as_str());

        if is_valid_tuple(left) && is_valid_tuple(right) {
            counts += 1;
        }
    }

    println!("part 2: {:?}", counts);
}

fn main() {
    let start = std::time::Instant::now();

    // let p = "/Users/zacswider/code/AOC2024/day4/test.txt";
    let p = "/Users/zacswider/code/AOC2024/day4/day4.txt";

    let word_search = load_input(p);
    part1(&word_search);
    part2(&word_search);

    let duration = start.elapsed();
    println!("time elapsed: {:?}", duration)
}
