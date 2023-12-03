use read_line::read_lines;
use regex::Regex;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    // Part one
    let mut total_1 = 0;
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let (game_number, playable) = is_game_playable(ip);

                if playable {
                    total_1 = total_1 + game_number;
                } 
            }
        }
    }

    println!("Total 1: {total_1}");

    // Part two 
    let mut total_2 = 0;
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let power = game_power(ip);
                total_2 = total_2 + power;
            }
        }
    }

    println!("Total 2: {total_2}");


}

fn is_game_playable(value: String) -> (i32, bool) {
    let reg = Regex::new(r"Game\s(?<number>\d.*)\:").unwrap();

    let game_number = reg.captures(&value).unwrap()["number"].parse::<i32>().unwrap();

    let game_string = reg.replace(&value.as_str(), "");

    let game_sets: Vec<&str> = game_string.split(';').collect();

    let mut game_valid = true;

    for set in game_sets {
        let moves: Vec<&str> = set.split(',').collect();

        for game_move in moves {
            let sanitized_move = game_move.trim();
            let game_move_item: Vec<&str> = sanitized_move.split(' ').collect();

            let amount = game_move_item.get(0).unwrap().parse::<i32>().unwrap();
            let color = game_move_item.get(1).unwrap().to_owned();

            if (color == "blue" && amount > 14)
                || (color == "red" && amount > 12)
                || (color == "green" && amount > 13)
            {
                game_valid = false;
            }
        }
    }

    return (game_number, game_valid);
}

#[test]
fn should_return_true_game_is_valid() {
    let value = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let result = is_game_playable(value.to_string());
    assert_eq!(result, (1, true));
}

#[test]
fn should_return_false_game_is_invalid() {
    let value = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let result = is_game_playable(value.to_string());
    assert_eq!(result, (3, false));
}

#[test]
fn should_return_true_game_is_valid_1() {
    let value = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let result = is_game_playable(value.to_string());
    assert_eq!(result, (3, false));
}

fn game_power(value: String) -> i32 {
    let reg = Regex::new(r"Game\s(?<number>\d.*)\:").unwrap();

    let game_string = reg.replace(&value.as_str(), "");

    let game_sets: Vec<&str> = game_string.split(';').collect();

    let mut blue_vector = Vec::new();
    let mut red_vector = Vec::new();
    let mut green_vector = Vec::new();

    for set in game_sets {
        let moves: Vec<&str> = set.split(',').collect();

        for game_move in moves {
            let sanitized_move = game_move.trim();
            let game_move_item: Vec<&str> = sanitized_move.split(' ').collect();

            let amount = game_move_item.get(0).unwrap().parse::<i32>().unwrap();
            let color = game_move_item.get(1).unwrap().to_owned();

            if color == "blue" { blue_vector.push(amount)}
            if color == "red" { red_vector.push(amount)}
            if color == "green" { green_vector.push(amount)}

        }
    }

    let blue_max = blue_vector.iter().max().unwrap();
    let red_max = red_vector.iter().max().unwrap();
    let green_max = green_vector.iter().max().unwrap();

    let power = (red_max * green_max) * blue_max;

    return power;
}

#[test]
fn should_return_right_power() {
    let value = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let result = game_power(value.to_string());
    assert_eq!(result, 48);
}

#[test]
fn should_return_right_power_2() {
    let value = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    let result = game_power(value.to_string());
    assert_eq!(result, 12);
}

#[test]
fn should_return_right_power_3() {
    let value = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let result = game_power(value.to_string());
    assert_eq!(result, 1560);
}
