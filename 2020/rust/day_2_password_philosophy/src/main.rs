struct ProcessedInput<'a>(&'a str, &'a str, &'a str);

fn clean_line(line: &str) -> ProcessedInput {
     let split = line.split_whitespace();
     return ProcessedInput(split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
}

fn validate_password(input: &ProcessedInput) -> bool {
    let char_to_check = input.1.chars().next().unwrap();
    let char_counts = input.0.split_whitespace();
    let min_char_count: u32 = char_counts.next().unwrap().to_string().parse().unwrap();
    let max_char_count: u32 = char_counts.next().unwrap().to_string().parse().unwrap();
    let password = input.2;
    let mut char_count = 0;
    print!("Checking {} for {}-{} {}s", password, min_char_count, max_char_count, char_to_check);
    for c in password.chars() {
        if c == char_to_check {
            char_count += 1;
        }
    }

    let mut result: bool = true;
    if  char_count > max_char_count || char_count < min_char_count {
        result = false;
    }
    print!(" ==> {}", result);
    return result
}

fn main() {
    let valid_passwords = std::fs::read_to_string("input.txt")
        .expect("file not found!")
        .lines()
        .map(|line| clean_line(line))
        .filter(|proc_in| validate_password(proc_in))
        .count();
    println!("{} passwords are valid!", valid_passwords);
}

