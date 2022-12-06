use std::str::Split;

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| line.split(',').map(|part| part.to_string()).collect())
        .map(|assigements: Vec<String>| {
            assigements // ["2-4","6-8"]
                .iter() // ["2-4"] -> [2, 4]
                .map(|assigement| {
                    assigement
                        .split('-')
                        .map(|p| p.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .map(|a| does_any_entry_contain_another(&a))
        // .map(does_any_entry_contain_another)
        .filter(|&x| x)
        .count() as u32;

    Option::Some(total)
}

fn does_any_entry_contain_another(assigements: &[Vec<usize>]) -> bool {
    let first = &assigements[0]; // [2, 4]
    let second = &assigements[1]; // [6, 8]

    match first[0].cmp(&second[0]){
        std::cmp::Ordering::Less => {
            if first[1] >= second[1] {
                return true
            }
        },
        std::cmp::Ordering::Equal => {
            return true
        },
        std::cmp::Ordering::Greater => {
            if second[1] >= first[1] {
                return true
            }
        },
    }

    false
}

fn does_any_entry_overlap(assigements: &[Vec<usize>]) -> bool {
    let first = &assigements[0]; // [6, 8]
    let second = &assigements[1]; // [2, 4]

    match first[0].cmp(&second[0]){
        std::cmp::Ordering::Less => {
            if first[1] < second[0] {
                return false
            }
            return true
        },
        std::cmp::Ordering::Equal => {
            return true
        },
        std::cmp::Ordering::Greater => {
            if first[0] > second[1] {
                return false
            }
        },
    }

    true
}


pub fn part_two(input: &str) -> Option<u32> {
    let total = input
    .lines()
    .map(|line| line.split(',').map(|part| part.to_string()).collect())
    .map(|assigements: Vec<String>| {
        assigements // ["2-4","6-8"]
            .iter() // ["2-4"] -> [2, 4]
            .map(|assigement| {
                assigement
                    .split('-')
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
    })
    .map(|a| does_any_entry_overlap(&a))
    // .map(does_any_entry_contain_another)
    .filter(|&x| x)
    .count() as u32;

Option::Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
