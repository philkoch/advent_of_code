fn get_file_contents_as_vec(file_name: &str) -> Vec<u32> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .map(|parsed| parsed.unwrap_or(0))
        .collect()
}

// Part 1
fn get_2_2020_summerands(numbers: &Vec<u32>, ex_result: &u32) -> () {
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            let add_result = numbers[i]+ numbers[j];
            if add_result == *ex_result {
                println!("Found {} and {} = {}", numbers[i], numbers[j], add_result);
                println!("Result is:  {}", numbers[i] * numbers[j]);
            }
        }
    }
}

// Part 2
fn get_3_2020_summerands(numbers: &Vec<u32>, ex_result: &u32) -> () {
    let length = numbers.len();
    for i in 0..length {
        for j in i+1..length {
            for k in j+1..length {
                let add_result = numbers[i]+ numbers[j] + numbers[k];
                if add_result == *ex_result {
                    println!("Found {}, {} and {} = {}", numbers[i], numbers[j], numbers[k], add_result);
                    println!("Result is:  {}", numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
}

fn main() {
    let numbers = get_file_contents_as_vec("src/input.txt");
    let expected_result_sum: u32 = 2020;
    println!("Searching 2 summerands in {} numbers that add up to {}", numbers.len(), expected_result_sum);
    get_2_2020_summerands(&numbers, &expected_result_sum);
    println!();
    println!("Searching 3 summerands in {} numbers that add up to {}", numbers.len(), expected_result_sum);
    get_3_2020_summerands(&numbers, &expected_result_sum);
}
