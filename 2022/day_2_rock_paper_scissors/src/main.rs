fn get_file_contents_as_vec(file_name: &str) -> Vec<String> {
    return std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|line| line.to_string())
        .collect();
}

#[derive(Debug, PartialEq, Eq)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}


fn create_move(move_char: &str) -> Move {
    if move_char == "A" || move_char == "X" {
        return Move::ROCK;
    }
    if move_char == "B" || move_char == "Y" {
        return Move::PAPER;
    }
    if move_char == "C" || move_char == "Z" {
        return Move::SCISSORS;
    } else {
        panic!("Unkown Move!");
    }
}

fn fake_move(move_char: &str, enemy_move: &Move) -> Move {
    if move_char == "Y" {
        // DRAW
        return match enemy_move {
            &Move::ROCK => Move::ROCK,
            &Move::SCISSORS => Move::SCISSORS,
            &Move::PAPER => Move::PAPER,
        }
    }
    if move_char == "X" {
        // LOSS
        return match enemy_move {
            &Move::ROCK => Move::SCISSORS,
            &Move::SCISSORS => Move::PAPER,
            &Move::PAPER => Move::ROCK,
        }
    }
    if move_char == "Z" {
        // WIN
        return match enemy_move {
            &Move::ROCK => Move::PAPER,
            &Move::SCISSORS => Move::ROCK,
            &Move::PAPER => Move::SCISSORS,
        }
    }
    panic!("Unknown move char {}", move_char);
}


fn get_win_value(own_move: &Move, enemy_move: &Move) -> u32 {
    let win = 6;
    let loss = 0;
    if own_move == enemy_move {
        return 3;
    }
    if own_move == &Move::ROCK && enemy_move == &Move::SCISSORS
    || own_move == &Move::PAPER && enemy_move == &Move::ROCK
    || own_move == &Move::SCISSORS && enemy_move == &Move::PAPER {
        return win
    }
    return loss;
}

fn get_move_value(own: &Move) -> u32 {
    match own {
     Move::ROCK => 1,
     Move::PAPER => 2,
     Move::SCISSORS => 3,
    }
}

fn evaluate_round(enemy: &str, own: &str) -> u32 {
    let own_move = create_move(own);
    let enemy_move = create_move(enemy);
    return get_win_value(&own_move, &enemy_move) + get_move_value(&own_move)
}

fn fake_round(enemy: &str, own: &str) -> u32 {
    let enemy_move = create_move(enemy);
    let own_move = fake_move(own, &enemy_move);
    return get_win_value(&own_move, &enemy_move) + get_move_value(&own_move)
}


fn part_one(puzzle_input: &Vec<String>) -> u32 {
    /*calculates the total points if everything goes according to plan*/
    let mut total_points = 0;
    for round in puzzle_input.iter() {
        let moves: Vec<&str> = round.split(" ").collect();
        total_points += evaluate_round(moves[0], moves[1]);
    }
    return total_points;
}

fn part_two(puzzle_input: &Vec<String>) -> u32 {
    let mut total_points = 0;
    for round in puzzle_input.iter() {
        let moves: Vec<&str> = round.split(" ").collect();
        total_points += fake_round(moves[0], moves[1]);
    }
    return total_points;
}

fn main() {
    let puzzle_input = get_file_contents_as_vec("src/input");
    println!("Part 1:");
    println!("-----------------");
    println!("What would your total score be if everything goes exactly according to your strategy guide?");
    println!();
    let part_one_answer = part_one(&puzzle_input);
    println!("Answer: {}", part_one_answer);

    println!("Part 2:");
    println!("-----------------");
    println!("Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?");
    println!();
    let part_two_answer = part_two(&puzzle_input);
    println!("Answer: {}", part_two_answer);


}
