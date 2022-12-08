fn get_file_contents_as_vec(file_name: &str) -> Vec<u32> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .map(|parsed| parsed.unwrap_or(0))
        .collect()
}

fn count_calories(puzzle_input: &Vec<u32>) -> Vec<u32> {
    /* adds up the calories for each elf and returns an
    unordered list of all calory-sums */
    let mut elves = vec![];
    let mut calorie_counter:u32 = 0;
    for line in puzzle_input.iter() {
        if line == &0 {
            // empty line means new elf
            elves.push(calorie_counter);
            calorie_counter = 0;
        } else {
            calorie_counter += line;
        }
    }
    return elves;
}
fn part_one(input: &Vec<u32>) -> u32 {
    let mut elves = count_calories(input);
    elves.sort();
    elves.reverse();
    return elves[0];
}

fn part_two(input: &Vec<u32>) -> u32 {
    let mut elves = count_calories(input);
    elves.sort();
    elves.reverse();
    elves.resize(3, 0);
    let mut calories = 0;
    for elf in elves.iter() {
        calories += elf;
    }
    return calories;
}

fn main() {
    let input = get_file_contents_as_vec("input");
    println!("Part 1:");
    println!("Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?");
    let part_one_answer = part_one(&input);
    println!("Answer: {}", part_one_answer);
    assert!(67622 == part_one_answer, "{} is the wrong answer!", part_one_answer);
    println!();
    println!("Part 2:");
    println!("Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?");
    let part_two_answer = part_two(&input);
    println!("Answer: {}", part_two_answer);
    assert!(201491 == part_two_answer, "{} is the wrong answer!", part_two_answer);
}
