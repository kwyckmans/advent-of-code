static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part_one(input: &Vec<String>) -> Option<u32> {
    let total: usize = input
        .iter()
        .map(|content| content.split_at(content.len() / 2))
        .map(|(first, second)| {
            find_common_item_generic(vec![first.to_string(), second.to_string()])
        })
        .map(|c| ALPHABET.find(c.unwrap()).unwrap() + 1)
        .sum();

    Option::Some(total as u32)
}

// Guaruanteed to have somnething in common
// TODO: Could do this recursively.
fn find_common_item_generic(rugsacks: Vec<String>) -> Option<char> {
    let rugsack = &rugsacks[0];

    for c in rugsack.chars() {
        let mut found = true;
        for rugsack in &rugsacks[1..] {
            if !rugsack.contains(c) {
                found = false;
                break;
            }
        }

        if found {
            return Option::Some(c);
        }
    }
    Option::None
}

pub fn part_two(input: &Vec<String>) -> Option<u32> {
    let mut total = 0;
    for n in (0..input.len() - 2).step_by(3) {
        let badge = find_common_item_generic(input[n..n + 3].to_vec()).unwrap();
        total += ALPHABET.find(badge).unwrap() + 1;
    }

    Option::Some(total as u32)
}

fn main() {
    let input = &advent_of_code::read_file_to_arr("inputs", 3);
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
        assert_eq!(part_two(&input), Option::Some(70));
    }
}
