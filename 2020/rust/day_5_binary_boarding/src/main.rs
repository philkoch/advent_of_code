use std::cmp;

fn load_puzzle_input(path: &str) -> Vec<String> {
	std::fs::read_to_string(path)
        .expect("file not found!")
        .lines()
		.map(|line| line.to_string())
		.collect()
}

fn find_seat(seat_def: &String, max_row: u32, max_seat: u32) -> (u32, u32) {
    let mut rows = (0, max_row);
    let mut seats = (0, max_seat);
    let mut row = 0;
    let mut seat = 0;
    for c in seat_def.chars() {
        match c {
            'F' => {
                rows = get_lower_half(rows.0, rows.1);
                row = rows.0
            }
            'B' => {
                rows = get_upper_half(rows.0, rows.1);
                row = rows.1
            },
            'L' => {
                seats = get_lower_half(seats.0, seats.1);
                seat = seats.0
            },
            'R' => {
                seats = get_upper_half(seats.0, seats.1);
                seat = seats.1
            }
            _ => continue
        }
    }
    (row, seat)
}

fn get_seat_id(row: u32, seat: u32) -> u32 {
    row * 8 + seat
}

fn get_lower_half(lower: u32, upper: u32) -> (u32, u32) {
    let half = lower + (upper - lower) / 2;
    (lower, half)
}

fn get_upper_half(lower: u32, upper: u32) -> (u32, u32) {
    let half = upper - (upper - lower) / 2;
     (half, upper)
}

fn get_max_seat_id(current_max: u32, seat_id: u32) -> u32 {
    cmp::max(current_max, seat_id)
}

fn main() {
    let input = load_puzzle_input("src/input");
    let mut max_seat_id = 0;
    for seat_def in input {
        let seat = find_seat(&seat_def, 127, 7);
        let seat_id = get_seat_id(seat.0, seat.1);
        max_seat_id = get_max_seat_id(max_seat_id, seat_id);
    }
    println!("PART_1:");
    println!("max seat id is {}", max_seat_id);
}

#[test]
fn test_find_seat_examples() {
    let input = String::from("FBFBBFFRLR");
    let seat = find_seat(&input, 127, 7);
    assert_eq!((44,5), seat);
    assert_eq!(357, get_seat_id(seat.0, seat.1));

    let input = String::from("BFFFBBFRRR");
    let seat = find_seat(&input, 127, 7);
    assert_eq!((70,7), seat);
    assert_eq!(567, get_seat_id(seat.0, seat.1));

    let input = String::from("FFFBBBFRRR");
    let seat = find_seat(&input, 127, 7);
    assert_eq!((14,7), seat);
    assert_eq!(119, get_seat_id(seat.0, seat.1));

    let input = String::from("BBFFBBFRLL");
    let seat = find_seat(&input, 127, 7);
    assert_eq!((102,4), seat);
    assert_eq!(820, get_seat_id(seat.0, seat.1));
}
