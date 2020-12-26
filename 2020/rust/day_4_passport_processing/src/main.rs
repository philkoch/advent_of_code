use regex::Regex;

#[derive(PartialEq, Debug, Default, Clone)]
struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiration_year: usize,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: usize
}

impl Passport {

    fn set_expiration_year(&mut self, year: String) {
        self.expiration_year = match usize::from_str_radix(&year, 10) {
            Ok(y) => {
                if y >= 2020 && y <= 2030 {
                    y
                } else {
                    0
                }
            },
            Err(_) => 0
        }
    }

    fn set_eye_color(&mut self, color: String) {
        self.eye_color = match color.as_str() {
            "amb"|"blu"|"brn"|"gry"|"grn"|"hzl"|"oth" => color,
            _ => "".to_string()
        }
    }

    fn set_birth_year(&mut self, year: String) {
        self.birth_year = match usize::from_str_radix(&year, 10) {
            Ok(y) => {
                if y >= 1920 && y <= 2002 {
                    y
                } else {
                    0
                }
            },
            Err(_) => 0
        }
    }

    fn set_issue_year(&mut self, year: String) {
        self.issue_year = match usize::from_str_radix(&year, 10) {
            Ok(y) => {
                if y >= 2010 && y <= 2020 {
                    y
                } else {
                    0
                }
            },
            Err(_) => 0
        }
    }

    fn set_country_id(&mut self, country_id: String) {
        self.country_id = usize::from_str_radix(&country_id, 10).unwrap();
    }

    fn set_height(&mut self, height: String) {
        let mut lower_bound = 0;
        let mut upper_bound = 0;
        let mut bound_len = 0;
        if height.ends_with("cm") {
            lower_bound = 150;
            upper_bound = 193;
            bound_len = 3;
        } else if height.ends_with("in") {
            lower_bound = 59;
            upper_bound = 76;
            bound_len = 2;
        } else {
            return
        }
        if height.len() != bound_len + 2 {
            return
        } else {
            self.height = match height[0..bound_len].parse::<u32>() {
                Ok(x) => {
                    if x >= lower_bound && x <= upper_bound {
                        height
                    } else {
                        "".to_string()
                    }
                }
                _ => "".to_string()
            }
        }
    }

    fn set_hair_color(&mut self, color: String) {
        let hair_regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        self.hair_color = match hair_regex.find(&color) {
            Some(_) => color,
            _ => "".to_string()
        }
    }

    fn set_passport_id(&mut self, pid: String) {
        let pid_regex= Regex::new("^[0-9]{9}$").unwrap();
        self.passport_id = match pid_regex.find(&pid) {
            Some(_) => pid,
            _ => "".to_string()
        }
    }

    fn validate(&self) -> bool {
        validate_num(&self.birth_year) &&
        validate_num(&self.issue_year) &&
        validate_num(&self.expiration_year) &&
        validate_str(&self.height) &&
        validate_str(&self.hair_color) &&
        validate_str(&self.eye_color) &&
        validate_str(&self.passport_id)
    }

    fn from(passport_str: String) -> Option<Passport> {
        let split_pass = passport_str.split_whitespace();
        let mut pp = Passport::default();
        for key_val in split_pass {
            let mut colon_val = key_val.split(":");
            let key = match colon_val.next() {
                Some(k) => k,
                None => panic!()
            };

            let value = match colon_val.next() {
                Some(v) => v.to_string(),
                None => panic!()
            };

            match key {
                "eyr" => pp.set_expiration_year(value),
                "ecl" => pp.set_eye_color(value),
                "hcl" => pp.set_hair_color(value),
                "pid" => pp.set_passport_id(value),
                "cid" => pp.set_country_id(value),
                "byr" => pp.set_birth_year(value),
                "iyr" => pp.set_issue_year(value),
                "hgt" => pp.set_height(value),
                _ => panic!()
            }
        }
        if pp.validate() {
            return Some(pp);
        }
        return None
    }
}

fn load_puzzle_input(path: &str) -> Vec<String> {
	std::fs::read_to_string(path)
        .expect("file not found!")
        .lines()
		.map(|line| line.to_string())
		.collect()
}

fn count_valid_passports(input: Vec<String>) -> u32 {
	let mut pp_str = "".to_owned();
	let mut pp_count: u32 = 0;
	for line in input {
		if !line.is_empty() {
			pp_str.push_str(&line);
			pp_str.push_str(" ");
		} else {
			if Passport::from(pp_str).is_some() {
				pp_count += 1
			}
			pp_str = "".to_owned();
		}
	}

    if !pp_str.is_empty() {
        if Passport::from(pp_str).is_some() {
            pp_count += 1
        }
    }

	return pp_count;
}


fn validate_num(num: &usize) -> bool {
    num > &0
}

fn validate_str(value: &String) -> bool {
    value != ""
}

fn main() {
	let puzzle_input = load_puzzle_input("src/input");
	println!("Found {} valid pp's", count_valid_passports(puzzle_input));
}

#[test]
fn test_pass_from() {
    let passport_str: String = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm");
    let passport: Passport = Passport {
            birth_year: 1937,
            issue_year: 2017,
            expiration_year: 2020,
            height: String::from("183cm"),
            hair_color: String::from("#fffffd"),
            eye_color: String::from("gry"),
            passport_id: String::from("860033327"),
            country_id: 147
    };
    assert_eq!(passport, Passport::from(passport_str).unwrap());

    let passport_str_2: String = String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
                                              hcl:#cfa07d byr:1929");
    assert!(Passport::from(passport_str_2).is_none());

    let passport_str_3: String = String::from("hcl:#ae17e1 iyr:2013
                                              eyr:2024
                                              ecl:brn pid:760753108 byr:1931
                                              hgt:179cm");
    let passport: Passport = Passport {
            birth_year: 1931,
            issue_year: 2013,
            expiration_year: 2024,
            height: String::from("179cm"),
            hair_color: String::from("#ae17e1"),
            eye_color: String::from("brn"),
            passport_id: String::from("760753108"),
            country_id: 0
    };
    assert_eq!(passport, Passport::from(passport_str_3).unwrap());

    let passport_str_4: String = String::from("hcl:#cfa07d eyr:2025 pid:166559648
                                              iyr:2011 ecl:brn hgt:59in");
    assert!(Passport::from(passport_str_4).is_none());
}

#[test]
fn test_puzzle_input() {
	let puzzle_input = load_puzzle_input("src/input_test");
	assert_eq!(2, count_valid_passports(puzzle_input));
}

#[test]
fn test_puzzle_input_invalid() {
	let puzzle_input = load_puzzle_input("src/input_test_invalid");
	assert_eq!(0, count_valid_passports(puzzle_input));
}


