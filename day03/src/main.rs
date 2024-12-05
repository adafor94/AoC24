use std::fs::read_to_string;
use std::i32;

use regex::Regex;

fn main() {
    let memory = read_to_string("input.txt").unwrap();

    // PART 1
    let part1:i32 = calc_sums(&memory);

    // PART 2
    let do_dont_re = Regex::new(r"(do\(\)|^)(?s)(.*?)(don't\(\))").unwrap();
    let part2:i32 = do_dont_re.captures_iter(memory.as_str()).map(|c| {
        let (_, [_, s, _]) = c.extract();
        calc_sums(&s.to_string())
    }).sum();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);

    assert_eq!(part1, 170778545);
    assert_eq!(part2, 82868252);
}

fn calc_sums(hay: &String) -> i32 {
    let mul_re = Regex::new(r"(mul\()([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    mul_re.captures_iter(hay.as_str()).map(|c| {
        let (_, [_, n1, n2]) = c.extract();
        n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()
    }).sum()
}
