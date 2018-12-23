use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let output = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string();

    println!("Part1 {}", output);
}

fn part2(input: &str) {
    let mut frequency: i32 = 0;
    let mut frequencies: HashSet<i32> = HashSet::new();

    loop {
        for line in input.lines() {
            frequency += line.parse::<i32>().unwrap();

            if frequencies.contains(&frequency) {
                println!("Part2 {}", frequency);
                return;
            }

            frequencies.insert(frequency);
        }
    }
} 
