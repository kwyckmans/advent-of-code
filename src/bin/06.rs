use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u32> {
    let mut buf= VecDeque::new();

    for (i, c) in input.char_indices(){
        assert!(buf.len() <= 4);

        if buf.len() < 4 {
            buf.push_back(c);
        } else if contains_duplicates(&buf) {
            println!("{}: {} {:?}", i, c, buf);
            buf.pop_front();
            buf.push_back(c);
        } else {
            println!("{}: {} {:?}", i, c, buf);
            return Some(i as u32);
        }
    }
    
    None
}

fn contains_duplicates(buf: &VecDeque<char>) -> bool {
    let mut seen = std::collections::HashSet::new();
    for c in buf {
        if seen.contains(c) {
            return true;
        }
        seen.insert(c);
    }
    false
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut buf= VecDeque::new();

    for (i, c) in input.char_indices(){
        assert!(buf.len() <= 14);

        if buf.len() < 14 {
            buf.push_back(c);
        } else if contains_duplicates(&buf) {
            println!("{}: {} {:?}", i, c, buf);
            buf.pop_front();
            buf.push_back(c);
        } else {
            println!("{}: {} {:?}", i, c, buf);
            return Some(i as u32);
        }
    }
    
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
