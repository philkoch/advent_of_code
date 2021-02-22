use std::collections::HashMap;
use std::{thread, time};

fn load_puzzle_input(path: &str) -> Vec<String> {
	std::fs::read_to_string(path)
        .expect("file not found!")
        .lines()
		.map(|line| line.to_string())
		.collect()
}

#[derive(Debug, Clone)]
struct Content {
	color: String,
	num: i32,
}

fn create_reverse_index(rules: &mut HashMap<String, Vec<Content>>, rule_def: &String) {
	let split_rule: Vec<&str> = rule_def.split(" bags contain ").collect();
	let parent = split_rule.iter().nth(0).unwrap().to_string();
	let contents = split_rule.iter().nth(1).unwrap();
	let contents_vec: Vec<&str> = contents.split(',').collect();
	for content in contents_vec {
		let mut words_split = content.split_whitespace();
		match words_split.next().unwrap().to_string().parse::<i32>() {
			Ok(num) => {
				let color = format!("{} {}", words_split.next().unwrap(), words_split.next().unwrap());
				let rule_content = Content {
					color: parent.clone(),
					num,
				};
				match rules.get_mut(&color) {
					Some(vec) => vec.push(rule_content),
					None => {
						let vec: Vec<Content> = vec!(rule_content);
						rules.insert(color, vec);
					}
				}
				//println!("map: {:#?}", rules);
			},
			Err(_e) => continue,
		}
	}
}

fn how_many_bags_contain(index: &mut HashMap<String, Vec<Content>>, color: &String) -> Vec<Content> {
	println!("Checking how many bags eventually contain {}", color);
	let mut distinct_colors: Vec<Content> = Vec::new();
	let mut color_val: Vec<Content> = match index.get_mut(color) {
		Some(vec) => {
			println!("Found {} parents for {}.", vec.len(), color);
            println!("{:?}", vec);
            vec.to_vec()
		},
		None => Vec::new()
	};
    distinct_colors.append(&mut color_val.to_vec());
    for color_child in color_val {
        println!("Checking child {}[{}]", &color_child.color, &color_child.num);
        distinct_colors.append(&mut how_many_bags_contain(index, &color_child.color));
    }
    distinct_colors
}

fn create_result(containing_bags: Vec<Content>) -> usize {
    let mut possible_bags: Vec<String>  = Vec::new();
    for bag in containing_bags {
        possible_bags.push(bag.color);
    }
    possible_bags.sort();
    println!("{:?}", possible_bags);
    possible_bags.dedup();
    println!("{:?}", possible_bags);
    possible_bags.len()

}

fn main() {
	let puzzle: Vec<String> = load_puzzle_input("src/input");
	let mut rule_map: HashMap<String, Vec<Content>> = HashMap::new();
	println!("Creating rules...");
	for line in puzzle {
		create_reverse_index(&mut rule_map, &line);
	}
	println!("Rules created.");
	let bags = how_many_bags_contain(&mut rule_map, &format!("shiny gold"));
    let result = create_result(bags);
    println!("Found {} distinct colors", result);
}

#[test]
fn test_example_data(){
   let mut test_input: Vec<String> = Vec::with_capacity(10);
   test_input.push("light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string());
   test_input.push("dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string());
   test_input.push("bright white bags contain 1 shiny gold bag.".to_string());
   test_input.push("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string());
   test_input.push("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string());
   test_input.push("dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string());
   test_input.push("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string());
   test_input.push("faded blue bags contain no other bags.".to_string());
   test_input.push("dotted black bags contain no other bags.".to_string());

	let mut rule_map: HashMap<String, Vec<Content>> = HashMap::new();
	println!("Creating test rules...");
	for line in test_input {
		create_reverse_index(&mut rule_map, &line);
	}
	println!("Test Rules created.");
	let bags = how_many_bags_contain(&mut rule_map, &format!("shiny gold"));
    let result = create_result(bags);
    assert_eq!(4, result);
}
