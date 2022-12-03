use advent_of_code::read_file_to_arr;

fn main() {
    let inputs = read_file_to_arr("inputs", 1);

    let (max_cals, total) = calc_total_cals_for_elves(&inputs);
    println!(
        "Elf with the most calories is carrying {} calories",
        max_cals
    );
    println!(
        "Top three elves with the most calories are carrying {} calories in total",
        total
    );
}

fn calc_total_cals_for_elves(elves: &Vec<String>) -> (i32, i32) {
    // let mut max = 0;
    let mut temp = 0;
    let mut totals: Vec<i32> = Vec::new();

    for elf in elves {
        if elf.is_empty() {
            totals.push(temp);
            temp = 0;
        } else {
            temp += elf.parse::<i32>().unwrap();
        }
    }

    totals.sort();
    let top_three = &totals[totals.len() - 3..totals.len()];
    (top_three[top_three.len() - 1], top_three.iter().sum())
}
