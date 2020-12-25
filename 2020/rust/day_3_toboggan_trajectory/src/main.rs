use std::convert::TryFrom;

/// converts the file into a Vec<String> in which each element
/// is one line from the file
fn load_puzzle_input(path: &str) -> Vec<String> {
	std::fs::read_to_string(path)
        .expect("file not found!")
        .lines()
		.map(|line| line.to_string())
		.collect()
}

/// returns true if the column's char of line == #, otherwise false
fn is_tree(line: &String, column: usize) -> bool {
	if line.chars().nth(column).unwrap() == '#' {
		//println!("Found tree!");
		return true;
	}
	return false
}

/// calculates the next column-index based on the line, current column and  step size
fn get_next_col_idx(line: &String, column: i32, step: i32) -> usize {
	let mut col_idx: i32 = column + step;
	let line_len: i32 = i32::try_from(line.len()).unwrap();
	// -1 here to account for zero based indices
	if col_idx > line_len-1 {
		col_idx = (line_len - col_idx).abs();
	}
 	return usize::try_from(col_idx).unwrap();
}

/// takes a puzzle and step definition input to count the number of 'trees' we step on
fn how_many_trees(puzzle: Vec<String>, right: i32, down: usize) -> u64 {
	// first column is not counted --> start-field
	let mut col_idx: i32 = 0;
	let mut trees: u64 = 0;
	// skip first down element because we're going need to search
	// the line that we're in AFTER the step was completed
	for line in puzzle.iter().step_by(down).skip(1) {
		let col_idx_us: usize = get_next_col_idx(line, col_idx, right);
		col_idx = col_idx_us as i32;
		//println!("Checking char {} in {}", col_idx_us, line);
		if is_tree(line, col_idx_us) {
			trees += 1
		}
	}
    return trees
}

fn main() {
	// Part 1
	let path = "src/input";
    let puzzle: Vec<String> = load_puzzle_input(path);
    let trees = how_many_trees(puzzle.clone(), 3, 1);
	println!("Found {} trees in puzzle from {}", trees, path);


    println!("Part Two:");
	// Part 2
	let trees1 = how_many_trees(puzzle.clone(), 1, 1);
	let trees2 = how_many_trees(puzzle.clone(), 3, 1);
	let trees3 = how_many_trees(puzzle.clone(), 5, 1);
	let trees4 = how_many_trees(puzzle.clone(), 7, 1);
	let trees5 = how_many_trees(puzzle.clone(), 1, 2);
    println!("multiplying trees ({},{},{},{},{})...",trees1, trees2, trees3, trees4, trees5);
    let multiplied_trees = trees1 * trees2 * trees3 * trees4 * trees5;
    println!("Result: {}", multiplied_trees);

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

	#[test]
	fn test_is_tree() {
		assert_eq!(true,  is_tree(&String::from("..#.."), 2));
		assert_eq!(false, is_tree(&String::from("#.#.#"), 1));
		assert_eq!(false, is_tree(&String::from("#.#.#"), 3));
		assert_eq!(true,  is_tree(&String::from("#.#.#"), 0));
		assert_eq!(true,  is_tree(&String::from("#.#.#"), 2));
		assert_eq!(true,  is_tree(&String::from("#.#.#"), 4));
	}

	#[test]
	fn test_get_next_col_idx() {
		assert_eq!(0, get_next_col_idx(&String::from("#.#.#"), 2, 3));
		assert_eq!(1, get_next_col_idx(&String::from("#.#.#"), 3, 3));
		assert_eq!(3, get_next_col_idx(&String::from("#.#.#"), 0, 3));
	}

	#[test]
	fn test_how_many_trees() {
		// example from https://adventofcode.com/2020/day/3
		let mut puzzample: Vec<String> = Vec::new();
		puzzample.push(String::from("..##......."));
		puzzample.push(String::from("#...#...#.."));
		puzzample.push(String::from(".#....#..#."));
		puzzample.push(String::from("..#.#...#.#"));
		puzzample.push(String::from(".#...##..#."));
		puzzample.push(String::from("..#.##....."));
		puzzample.push(String::from(".#.#.#....#"));
		puzzample.push(String::from(".#........#"));
		puzzample.push(String::from("#.##...#..."));
		puzzample.push(String::from("#...##....#"));
		puzzample.push(String::from(".#..#...#.#"));
		assert_eq!(7, how_many_trees(puzzample.clone(), 3, 1));
		// Part 2 tests
		assert_eq!(2, how_many_trees(puzzample.clone(), 1, 1));
		assert_eq!(3, how_many_trees(puzzample.clone(), 5, 1));
		assert_eq!(4, how_many_trees(puzzample.clone(), 7, 1));
		assert_eq!(2, how_many_trees(puzzample.clone(), 1, 2));
	}

}
