pub fn part_one(input: &Vec<String>) -> Option<u32> {
    for contents in input {
        let first_compartiment = &contents[0..contents.len() / 2];
        let second_compartiment = &contents[contents.len() / 2..];

        println!("first compartiment contents: {}, second compartiment contents: {}", first_compartiment, second_compartiment);

        let shared_item = find_common_item(first_compartiment, second_compartiment);
        println!("Shared item {}", shared_item.unwrap());
        println!("Shared item ASCII {}", shared_item.unwrap() as u32);

        // This doesn't work since uppercase ASCII comes before lowercase ASCII, while in the exercise, the values are
        // higher.
        println!("Common item: {}", shared_item.unwrap() as u32 - 96);
    }

    Option::Some(0)
}

fn find_common_item(first: &str, second: &str) -> Option<char> {
    assert!(first.len() == second.len());

    for (i, c) in first.chars().enumerate() {
        if second.contains(c) {
            return Option::Some(c);
        }
    }

    println!("Could not find common item");

    Option::None
}

pub fn part_two(input: &Vec<String>) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file_to_arr("examples", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file_to_arr("examples", 3);
        assert_eq!(part_one(&input), Option::Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file_to_arr("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
