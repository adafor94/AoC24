use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let lines = read_lines("input.txt");
    let mut row1: Vec<i32> = vec![];
    let mut row2: Vec<i32> = vec![];
    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        let mut words = line.split_whitespace();
        row1.push(words.next().unwrap().parse::<i32>().unwrap());
        row2.push(words.next().unwrap().parse::<i32>().unwrap());
    }

    let mut m: HashMap<i32, i32> = HashMap::new();
    for x in &row2 {
        *m.entry(*x).or_default() += 1;
    }

    for elem1 in &row1 {
        match m.get(elem1) {
        Some(occurances) => part2 += *elem1 * *occurances,
        None => (),
        }
    }

    row1.sort();
    row2.sort();
    for it in row1.iter().zip(row2.iter()) {
        let (a, b) = it;
        part1 += (a-b).abs();
    }

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
