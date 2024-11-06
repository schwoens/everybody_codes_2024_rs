use std::fs::read_to_string;

fn main() {
    let input = read_to_string("quest2/inputs/input1.txt").expect("error reading input file");
    println!("Part 1: {}", part1(&input));
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input =
            read_to_string("inputs/test_input1.txt").expect("error reading test input file");
        assert_eq!(part1(&test_input), 4);
    }
}
