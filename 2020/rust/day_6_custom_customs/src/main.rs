fn load_puzzle_input(path: &str) -> Vec<String> {
	std::fs::read_to_string(path)
        .expect("file not found!")
        .lines()
		.map(|line| line.to_string())
		.collect()
}

fn get_yes_answers(puzzle: Vec<String>) -> u32 {
    let mut group_answers: Vec<char> = Vec::with_capacity(30);
    let mut yes_answers = 0;
    for line in puzzle {
        if line.is_empty() {
            yes_answers += group_answers.len();
            group_answers.clear();
        } else {
            add_char_to_group_answer(&mut group_answers, line);
        }
    }
    if group_answers.len() > 0 {
        // group_answers was not cleared at the end because there
        // was no newline at the end of the input
        yes_answers += group_answers.len();
    }
    yes_answers as u32
}

fn add_char_to_group_answer(group_answers: &mut Vec<char>, line: String) {
    for c in line.chars() {
        if !group_answers.contains(&c) {
            group_answers.push(c);
        }
    }
}

fn main() {
    let input = load_puzzle_input("src/input");
    let sum_yes_answers = get_yes_answers(input);
    println!("PART_1");
    println!("{} yes answers found", &sum_yes_answers);
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
