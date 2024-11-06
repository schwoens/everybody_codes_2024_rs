use core::panic;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("quest1/inputs/input1.txt").expect("error reading input file");
    println!("Part 1: {}", part1(&input));
    let input = read_to_string("quest1/inputs/input2.txt").expect("error reading input file");
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let input = input.trim();
    for c in input.chars() {
        sum += match c {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            _ => panic!("invalid character reached"),
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let input = input.trim();

    let mut chars = input.chars();

    while let Some(c1) = chars.next() {
        // we assert that the input always has a length divisible by 2
        let c2 = chars.next().expect("end of input reached");

        sum += get_potion_cost(c1);
        sum += get_potion_cost(c2);

        if c1 != 'x' && c2 != 'x' {
            sum += 2;
        }
    }
    sum
}

fn get_potion_cost(c: char) -> i32 {
    match c {
        'A' | 'x' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => panic!("invalid character reached"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input =
            read_to_string("inputs/test_input1.txt").expect("error reading test input file");
        assert_eq!(part1(&test_input), 5);
    }

    #[test]
    fn part2_works() {
        let test_input =
            read_to_string("inputs/test_input2.txt").expect("error reading test input file");
        assert_eq!(part2(&test_input), 28);
    }
}
