use read_line::read_lines;
use std::collections::HashMap;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    // Parse the a line
    // keep a the coordinates of a group of digits
    // keep a the coordinates of a special character
    // lookup which group of digits shares the same position -1 / +1
    // digits: [1,3] special_char: [4,4] --> diagonal detected
    //  - Previous line
    //  - Current line

    let mut previous_line: Vec<char> = Vec::new();

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let current_line: Vec<char> = ip.chars().collect();

                if previous_line.len() == 0 {
                    previous_line = current_line;
                    continue;
                }

                get_adjacents(&current_line, &previous_line);
            }
        }
    }
}

/**
 *  Previous line = { "1,3" => "456", "6,8" => "114" }
 *  Current line = { "4,4" => "*" }
 * */
fn get_adjacents(curr: &Vec<char>, prev: &Vec<char>) -> Vec<i32> {
    let mut adjacents = Vec::new();

    let mut start_coor = 0;
    let mut buffer = String::new();

    for (idx, char) in curr.iter().enumerate() {
        if *char == '.' {
            if start_coor != 0 {
                let chars_lookup = prev.get((start_coor - 1)..(idx + 1)).unwrap();

                for char_lookup in chars_lookup {
                    if (char_lookup.is_digit(10)) || (*char_lookup == '.') {
                        continue;
                    }
                    adjacents.push(buffer.clone().parse::<i32>().unwrap());
                }
                start_coor = 0;
                buffer = String::new();
            }

            continue;
        }


        if start_coor == 0 {
            start_coor = idx + 1;
        }

        buffer.push(char.to_owned());
    }

    println!("{adjacents:?}");

    return adjacents;
}

#[test]
fn should_return_adjacents() {
    let prev: Vec<char> = "467..114..".chars().collect();
    let curr: Vec<char> = "...*......".chars().collect();
    let result = get_adjacents(&curr, &prev);
    assert_eq!(result, [467]);
}

#[test]
fn should_return_adjacents_1() {
    let prev: Vec<char> = "...*......".chars().collect();
    let curr: Vec<char> = "..35..633.".chars().collect();
    let result = get_adjacents(&curr, &prev);
    assert_eq!(result, [35]);
}
