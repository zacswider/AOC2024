use std::io::BufRead;

fn load_input(input_path: &str) -> Vec<Vec<char>> {
    let file = std::fs::File::open(input_path).expect("Failed to read input file");
    let reader = std::io::BufReader::new(file);
    let mut lines: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line: Vec<char> = line.expect("Failed to read line").chars().collect();
        lines.push(line);
    }
    lines
}

fn move_guard() {}

fn find_guard(map: &Vec<Vec<char>>) -> (usize, usize) {
    let guard_orientations = ["^", "<", ">", "v"];
    let mut guard_pos: (usize, usize) = (0 as usize, 0 as usize);
    for (row_idx, line) in map.iter().enumerate() {
        for (col_idx, char_val) in line.iter().enumerate() {
            let mut buf = [0; 4];
            let encoded_char: &str = char_val.encode_utf8(&mut buf);
            if guard_orientations.contains(&encoded_char) {
                guard_pos.0 = row_idx;
                guard_pos.1 = col_idx;
            }
        }
    }
    guard_pos
}

fn part1(map: &Vec<Vec<char>>) {
    let position = find_guard(&map);
    println!("Found guard at {:?}", position);
}

fn main() {
    let start = std::time::Instant::now();
    // let p = "/Users/zacswider/code/AOC2024/day6/day6.txt";
    let p = "/Users/zacswider/code/AOC2024/day6/test.txt";
    let map = load_input(p);
    for line in map.iter() {
        println!("{:?}", &line);
    }
    part1(&map);

    let duration = start.elapsed();
    println!("time elapsed {:?}", duration);
}
