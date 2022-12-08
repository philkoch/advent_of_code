fn get_file_contents_as_vec(file_name: &str) -> Vec<u32> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .map(|parsed| parsed.unwrap_or(0))
        .collect()
}

fn main() {
    let input = get_file_contents_as_vec("input");
    let mut elves = vec![];
    let mut calorie_counter:u32 = 0;
    for line in input.iter() {
        if line == &0 {
            if elves.len() > 0 && calorie_counter > elves[0] {
                elves.insert(0, calorie_counter);
            }
            elves.push(calorie_counter);
            calorie_counter = 0;
        } else {
            calorie_counter += line;
        }
    }
    println!("Most calories: {}", elves[0])
}
