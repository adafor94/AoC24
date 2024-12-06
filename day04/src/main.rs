use std::{fs::read_to_string, usize};
use itertools::Itertools;

fn main() {
    let matrix = read_to_string("input.txt").unwrap().lines().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let (part1, part2): (i32, i32) = (0..matrix.len()).cartesian_product(0..matrix[0].len())
        .map(|(i,j)| (check_part1(&matrix, i, j), check_part2(&matrix, i, j)))
        .fold( (0,0), | (a1,b1), (a2,b2) | (a1+a2, b1+b2));

    println!("Part1 {}", part1);
    println!("Part2 {}", part2);
}

fn check_part2(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    // Bounds
    if i > 0 && i < matrix.len() - 1 && j > 0 && j < matrix[0].len() - 1 {
        if matrix[i][j] == 'A' &&
           ((matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S') || (matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M')) &&
           ((matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S') || (matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M')) {
            return 1;
        }
    }
    0
}

fn check_part1(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    fn check_dir(matrix: &Vec<Vec<char>>, mut i: i32, mut j:i32, dir: (i32,i32)) -> i32 {
        for next_char in vec!['M', 'A', 'S'] {
            i += dir.0;
            j += dir.1;

            if i < 0 || j < 0 || i as usize > matrix.len() -1 || j as usize > matrix[0].len() -1 {
                return 0;
            }

            let i_u:usize = i.try_into().unwrap();
            let j_u:usize = j.try_into().unwrap();
            if matrix[i_u][j_u] != next_char {
                return 0;
            }
        }
        return 1;
    }

    let c = matrix[i][j];
    if c == 'X' {
        [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)].iter().map(| dir | check_dir(matrix, i as i32, j as i32, *dir)).sum()
    } else {
        0
    }
}
