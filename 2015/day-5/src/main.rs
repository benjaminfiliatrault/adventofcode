use std::{collections::{HashMap, HashSet}, borrow::Borrow};

use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {

    let mut nice_counter = 0;
    let mut bonus_nice_counter = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let is_naugthy = is_it_naugthy(ip.clone());
                let bonus_is_naugthy = is_it_naugthy_bonus(ip.clone());

                if !is_naugthy {
                    nice_counter += 1;
                }

                if !bonus_is_naugthy {
                    bonus_nice_counter += 1;
                }

           }
        }
    }
    println!("{:}", nice_counter);
    println!("{:}", bonus_nice_counter);
}

fn is_it_naugthy_bonus(text: String) -> bool {
    let mut stage_1 = true;
    let mut stage_2 = true;

    // It contains at least one letter which repeats with exactly one letter between them
    let iter: Vec<char> = text.chars().collect();

    for (idx, char) in iter.iter().enumerate() {
        if let Some(next_char) = iter.get(idx + 2){
            println!("{:?}", char);
            println!("{:?}", next_char);

            stage_1 = !(char == next_char);
            if !stage_1 {
                break;
            }
        };

    }
 
    if stage_1 { return stage_1 };


    println!("{:?}", stage_1);

    for idx in 0..text.len() - 3 { 
        if *(&text[idx + 2..].contains(&text[idx..idx + 2])) { 
            stage_2 = false; break;
        }
    } 

    return stage_2;
}

#[test]
fn bonus_should_not_be_nauthy() {
    let input = "qjhvhtzxzqqjkmpb";

    let result = is_it_naugthy_bonus(input.to_string());

    assert_eq!(result, false);
}

#[test]
fn bonus_should_not_be_nauthy_2() {
    let input = "xxyxx";

    let result = is_it_naugthy_bonus(input.to_string());

    assert_eq!(result, false);
}

#[test]
fn bonus_should_be_nauthy() {
    let input = "uurcxstgmygtbstg";

    let result = is_it_naugthy_bonus(input.to_string());

    assert_eq!(result, true);
}

#[test]
fn bonus_should_be_nauthy_2() {
    let input = "ieodomkazucvgmuy";

    let result = is_it_naugthy_bonus(input.to_string());

    assert_eq!(result, true);
}

fn is_it_naugthy(text: String) -> bool {
    let mut stage_1 = true;
    let mut stage_2 = true;
    let mut stage_3 = true;


    if text.contains("ab") {
        return stage_1;
    }

    if text.contains("cd") {
        return stage_1;
    }

    if text.contains("pq") {
        return stage_1;
    }
    if text.contains("xy") {
        return stage_1;
    }

    let mut iter = text.chars().peekable();
    while let Some(character) = iter.next() {
         if let Some(next_char) = iter.peek() {
            stage_2 = !(*next_char == character);
        }

         if !stage_2 {
            break;
         }
    };

    if stage_2 {
        return stage_2;
    }

    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let mut vowels_counter = 0;

    let mut iter2 = text.chars().peekable();
    // Check for one letter that is present twice
    while let Some(character) = iter2.next() {
        if vowels.contains(&character) {
            vowels_counter += 1
        }
    }

    stage_3 = vowels_counter < 3;

    return stage_3;
}

#[test]
fn should_not_be_nauthy_has_all_requirements() {
    let input = "ugknbfddgicrmopn";

    let is_naugthy = is_it_naugthy(input.to_string());

    assert_eq!(is_naugthy, false);
}

#[test]
fn should_not_be_nauthy_has_three_vowels_and_double_letter() {
    let input = "aaa";

    let is_naugthy = is_it_naugthy(input.to_string());

    assert_eq!(is_naugthy, false);
}

#[test]
fn should_be_nauthy() {
    let input = "jchzalrnumimnmhp";

    let is_naugthy = is_it_naugthy(input.to_string());

    assert_eq!(is_naugthy, true);
}

#[test]
fn should_be_nauthy_2() {
    let input = "haegwjzuvuyypxyu";

    let is_naugthy = is_it_naugthy(input.to_string());

    assert_eq!(is_naugthy, true);
}

#[test]
fn should_be_nauthy_3() {
    let input = "dvszwmarrgswjxmb";

    let is_naugthy = is_it_naugthy(input.to_string());

    assert_eq!(is_naugthy, true);
}
