use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    part1(&input);
}

fn part1(input: &str) {
    let (mut two, mut three) = (0, 0);

    for line in input.lines() {
        let mut characters: Vec<&str> = line.split("").collect();
        let (mut has_two, mut has_three) = (false, false);

        loop {
            if characters.len() == 0 {
                break;
            }

            let character: &str = characters[0];
            let matches: Vec<&str> = line.matches(&character).collect();

            characters.retain(|c| c != &character);

            if !has_two && matches.len() == 2 {
                two += 1;
                has_two = true;
                continue;
            }

            if !has_three && matches.len() == 3 {
                three += 1;
                has_three = true;
                continue;
            }
        }
    }

    println!("{}", two * three)


}
