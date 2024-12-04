use std::fs::read_to_string;

fn main() {
    let (part1, part2) = read_to_string("input.txt").unwrap().lines().map(String::from)
        .fold((0,0), | acc, x | {
            let mut diff: Vec<i32> = x.split_whitespace().collect::<Vec<_>>().windows(2)
                .map(|a| a[1].parse::<i32>().unwrap() - a[0].parse::<i32>().unwrap()).collect();
            let res = check(&mut diff);
            (acc.0 + res.0, acc.1 + res.1)
        });

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn check(diff: &mut Vec<i32>) -> (i32, i32) {
    let mut part1 = 1;
    let mut sign = diff[0] > 0;
    for i in 0 .. diff.len() {
        let elem = diff[i];
        if ! (sign == (elem > 0) && elem.abs() > 0 && elem.abs() < 4) {
            if part1 == 0 {
                return (0,0);
            }

            part1 = 0;
            if i == 0 {
                sign = diff[i+1] > 0;
            } else if i < (diff.len() - 1) {
                diff[i+1] = diff[i-1] + diff[i];
            }
        }
    }
    return (part1, 1);
}
