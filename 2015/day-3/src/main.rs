use std::collections::HashSet;

use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {

    let mut result: usize = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                result = awesome_bonus_function(ip)                
                
           }
        }
    }

    println!("{:}", result);
}



fn awesome_function(directions: String) -> usize {
    let mut iter = directions.chars(); 

    let mut coordinate = vec![0, 0];
    let mut visited = HashSet::new();

    visited.insert(coordinate.clone());

    while let Some(char) = iter.next() {
        if char == '^' {
            coordinate[1] += 1;
        }

        if char == 'v' {
            coordinate[1] -= 1;
        }

        if char == '>' {
            coordinate[0] += 1;
        }

        if char == '<' {
            coordinate[0] -= 1;
        }

        visited.insert(coordinate.clone());
    }

    return visited.len();
}

fn awesome_bonus_function(directions: String) -> usize {
    let mut iter = directions.chars().enumerate(); 

    let mut coordinate_santa = vec![0, 0];
    let mut coordinate_robot = vec![0, 0];
    
    let mut visited = HashSet::new();

    visited.insert(coordinate_santa.clone());

    while let Some((idx, char)) = iter.next() {
        if idx % 2 == 0 {

            if char == '^' {
                coordinate_santa[1] += 1;
            }

            if char == 'v' {
                coordinate_santa[1] -= 1;
            }

            if char == '>' {
                coordinate_santa[0] += 1;
            }

            if char == '<' {
                coordinate_santa[0] -= 1;
            }

            visited.insert(coordinate_santa.clone());
            continue;
        }
            if char == '^' {
                coordinate_robot[1] += 1;
            }

            if char == 'v' {
                coordinate_robot[1] -= 1;
            }

            if char == '>' {
                coordinate_robot[0] += 1;
            }

            if char == '<' {
                coordinate_robot[0] -= 1;
            }

            visited.insert(coordinate_robot.clone());
         


    }

    return visited.len();


}


#[test]
fn should_deliver_to_two_houses() {
    let directions = ">";

    let result = awesome_function(directions.to_string());

    assert_eq!(result, 2);
}

#[test]
fn should_deliver_to_four_houses() {
     let directions = "^>v<";

    let result = awesome_function(directions.to_string());

    assert_eq!(result, 4);
}

#[test]
fn should_deliver_to_two_houses_again() {
    let directions = "^v^v^v^v^v";

    let result = awesome_function(directions.to_string());

    assert_eq!(result, 2);
}

#[test]
fn should_deliver_bonues_to_two_houses() {
    let directions = "^v";

    let result = awesome_bonus_function(directions.to_string());

    assert_eq!(result, 3);
}

#[test]
fn should_deliver_bonus_to_four_houses() {
     let directions = "^>v<";

    let result = awesome_bonus_function(directions.to_string());

    assert_eq!(result, 3);
}

#[test]
fn should_deliver_bonus_to_two_houses_again() {
    let directions = "^v^v^v^v^v";

    let result = awesome_bonus_function(directions.to_string());

    assert_eq!(result, 11);
}


