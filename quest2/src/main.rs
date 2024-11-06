use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("quest2/inputs/input1.txt").expect("error reading input file");
    println!("Part 1: {}", part1(&input));
    let input = read_to_string("quest2/inputs/input2.txt").expect("error reading input file");
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (words, helmet) = input.split_once("\n\n").unwrap();
    let (_, words) = words.split_once(":").unwrap();

    let mut word_sum = 0;

    for word in words.split(",") {
        word_sum += helmet.match_indices(word).count();
    }
    word_sum
}

fn part2(input: &str) -> usize {
    let input = input.trim();

    let (words, shield) = input.split_once("\n\n").unwrap();
    let (_, words) = words.split_once(":").unwrap();

    let mut symbol_sum = 0;
    let mut symbol_indicies = HashSet::new();

    for line in shield.split("\n") {
        symbol_indicies.clear();
        for word in words.split(",") {
            for (index, _) in line.match_indices(word) {
                for i in 0..word.chars().count() {
                    symbol_indicies.insert(index + i);
                }
            }

            let reverse_line = line.chars().rev().collect::<String>();
            for (index, _) in reverse_line.match_indices(word) {
                for i in 0..word.chars().count() {
                    symbol_indicies.insert(reverse_line.len() - 1 - index - i);
                }
            }
        }
        symbol_sum += symbol_indicies.len();
    }
    symbol_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input =
            read_to_string("inputs/test_input1.txt").expect("error reading test input file");
        assert_eq!(part1(&test_input), 4);
    }

    #[test]
    fn part2_works() {
        let test_input =
            read_to_string("inputs/test_input2.txt").expect("error reading test input file");
        assert_eq!(part2(&test_input), 42);
    }
}
