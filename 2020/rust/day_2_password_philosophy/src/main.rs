struct ProcessedInput<'a>(&'a str, &'a str, &'a str);

fn clean_line(line: &str) -> ProcessedInput {
     let mut split = line.split_whitespace();
     return ProcessedInput(split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
}

// Part 1
fn validate_password_char_count(input: &ProcessedInput) -> bool {
    let char_to_check = input.1.chars().next().unwrap();
    let mut char_counts = input.0.split('-');
    let min_char_count: u32 = char_counts.next().unwrap().to_string().parse().unwrap();
    let max_char_count: u32 = char_counts.next().unwrap().to_string().parse().unwrap();
    let password = input.2;
    let mut char_count = 0;
    print!("Checking {} for {}-{} {}'s", password, min_char_count, max_char_count, char_to_check);
    for c in password.chars() {
        if c == char_to_check {
            char_count += 1;
        }
    }

    let mut result: bool = true;
    if  char_count > max_char_count || char_count < min_char_count {
        result = false;
    }
    println!(" ==> {}", result);
    return result
}
// Part 2
fn validate_password_char_indices(input: &ProcessedInput) -> bool {
    let char_to_check = input.1.chars().next().unwrap();
    let mut char_positions = input.0.split('-');
    let first: usize = char_positions.next().unwrap().to_string().parse().unwrap();
    let second: usize = char_positions.next().unwrap().to_string().parse().unwrap();
    let password = input.2;
    print!("Checking {} for {}-{} {}'s", password, first, second, char_to_check);

    let mut result: bool = true;
    if password.chars().nth(first-1).unwrap() == char_to_check &&
       password.chars().nth(second-1).unwrap() == char_to_check {
            result = false;
    }
    if password.chars().nth(first-1).unwrap() != char_to_check &&
       password.chars().nth(second-1).unwrap() != char_to_check {
            result = false;
    }
    println!(" ==> {}", result);
    return result
}

fn main() {
    let part1 = std::fs::read_to_string("src/input.txt")
        .expect("file not found!")
        .lines()
        .map(|line| clean_line(line))
        .filter(|proc_in| validate_password_char_count(proc_in))
        .count();
    println!();
    println!("{} passwords are valid by count! -- PART1", part1);

    // TODO: can the reading of the file be moved to its own function?
    let part2 = std::fs::read_to_string("src/input.txt")
        .expect("file not found!")
        .lines()
        .map(|line| clean_line(line))
        .filter(|proc_in| validate_password_char_indices(proc_in))
        .count();
    println!();
    println!("{} passwords are valid by count! -- PART2", part2);
}

