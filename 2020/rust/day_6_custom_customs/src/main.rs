use std::collections::HashMap;

fn load_puzzle_input(path: &str) -> Vec<String> {
	std::fs::read_to_string(path)
        .expect("file not found!")
        .lines()
		.map(|line| line.to_string())
		.collect()
}

fn get_yes_answers(puzzle: &Vec<String>) -> u32 {
    let mut group_answers: Vec<char> = Vec::with_capacity(30);
    let mut yes_answers = 0;
    for line in puzzle {
        if line.is_empty() {
            yes_answers += group_answers.len();
            group_answers.clear();
        } else {
            add_char_to_group_answer_if_not_exists(&mut group_answers, line.to_string());
        }
    }
    if group_answers.len() > 0 {
        // group_answers was not cleared at the end because there
        // was no newline at the end of the input
        yes_answers += group_answers.len();
    }
    yes_answers as u32
}

fn add_char_to_group_answer_if_not_exists(group_answers: &mut Vec<char>, line: String) {
    for c in line.chars() {
        if !group_answers.contains(&c) {
            group_answers.push(c);
        }
    }
}

fn get_group_yes_answers(puzzle: &Vec<String>) -> u32 {
    let mut group_answers: HashMap<char, u32> = HashMap::new();
    let mut group_size = 0;
    let mut group_yes_answers = 0;

    for line in puzzle {
        if line.is_empty() {
            // reset parameters for next group
            group_yes_answers += count_answers(&group_answers, &group_size);
            group_size = 0;
            group_answers.clear();
        } else {
            add_char_to_group_answer(&mut group_answers, line);
            group_size += 1;
        }
    }
    if group_answers.len() > 0 {
        // group_answers was not cleared at the end because there
        // was no newline at the end of the input
        group_yes_answers += count_answers(&group_answers, &group_size);
    }
    group_yes_answers as u32
}

fn count_answers(group_answers: &HashMap<char,u32>, group_size: &u32) -> u32 {
    group_answers.iter()
        .filter(|(_key, value)| *value == group_size)
        .count() as u32

}

fn add_char_to_group_answer(group_answers: &mut HashMap<char,u32>, line: &String){
    for c in line.chars() {
        if !group_answers.contains_key(&c) {
            group_answers.insert(c, 0);
        }
        group_answers.insert(c, group_answers.get(&c).unwrap()+1);
    }
}

fn main() {
    let input = load_puzzle_input("src/input");
    let sum_yes_answers = get_yes_answers(&input);
    println!("PART_1");
    println!("{} yes answers found", &sum_yes_answers);

    let sum_group_yes_answers = get_group_yes_answers(&input);
    println!("PART_2");
    println!("{} questions found that the complete group answered with yes", &sum_group_yes_answers);
}

#[test]
fn test_part_1_example() {
    let mut input: Vec<String> = Vec::new();
    input.push(String::from("abc"));
    input.push(String::from(""));
    input.push(String::from("a"));
    input.push(String::from("b"));
    input.push(String::from("c"));
    input.push(String::from(""));
    input.push(String::from("ab"));
    input.push(String::from("ac"));
    input.push(String::from(""));
    input.push(String::from("a"));
    input.push(String::from("a"));
    input.push(String::from("a"));
    input.push(String::from("a"));
    input.push(String::from(""));
    input.push(String::from("b"));

    assert_eq!(11, get_yes_answers(input));
}
