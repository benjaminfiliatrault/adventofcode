use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let mut total_1 = 0;

    // Part one
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let result = extract_numbers(ip);
                total_1 = total_1 + result;
            }
        }
    }

    println!("First part result");
    println!("{total_1}\n");

    let mut total_2 = 0;

    // Part Two
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let result = extract_numbers_two(ip, None, true);
                total_2 = total_2 + result;
            }
        }
    }

    println!("Second part result");
    println!("{total_2}");
}

fn extract_numbers(value: String) -> i32 {
    let mut numbers = Vec::new();

    for char in value.chars() {
        if char.is_numeric() {
            numbers.push(char.to_digit(10).unwrap());
        }
    }

    let first = numbers.first().unwrap_or(&0);
    let last = numbers.last().unwrap_or(&0);

    let numbers = format!("{first}{last}");

    return numbers.parse::<i32>().unwrap_or(0);
}

#[test]
fn should_return_first_n_last_number() {
    let value = "1abc2".to_string();
    let result = extract_numbers(value);
    assert_eq!(result, 12);
}

#[test]
fn should_return_only_first_n_last_number() {
    let value = "a1b2c3d4e5f".to_string();
    let result = extract_numbers(value);
    assert_eq!(result, 15);
}

#[test]
fn should_return_the_only_number_twice() {
    let value = "treb7uchet".to_string();
    let result = extract_numbers(value);
    assert_eq!(result, 77);
}

fn extract_numbers_two(value: String, numbers: Option<Vec<i32>>, is_first_pass: bool) -> i32 {
    let mut cloned_string = value.clone();
    let mut cloned_numbers = Vec::new();

    if let Some(nbs) = numbers {
        cloned_numbers = nbs.clone();
    }

    // Fancy replacement on first pass only
    if is_first_pass {    
        cloned_string = cloned_string.replace("one", "one1one");
        cloned_string = cloned_string.replace("two", "two2two");
        cloned_string = cloned_string.replace("three", "three3three");
        cloned_string = cloned_string.replace("four", "four4four");
        cloned_string = cloned_string.replace("five", "five5five");
        cloned_string = cloned_string.replace("six", "six6six");
        cloned_string = cloned_string.replace("seven", "seven7seven");
        cloned_string = cloned_string.replace("eight", "eight8eight");
        cloned_string = cloned_string.replace("nine", "nine9nine");
    }

    // Check if the first char is a digit
    let char = cloned_string.chars().next().unwrap_or('0');
    if char.is_numeric() && char != '0' {
        cloned_numbers.push(char.to_digit(10).unwrap() as i32);
        cloned_string = cloned_string.replacen(char, "", 1);
        return extract_numbers_two(cloned_string.clone(), Some(cloned_numbers.clone()), false);
    }


    if cloned_string.len() > 0 {
        cloned_string.remove(0);
        return extract_numbers_two(cloned_string, Some(cloned_numbers.clone()), false);
    }

    let first = cloned_numbers.first().unwrap();
    let last = cloned_numbers.last().unwrap();

    let res = format!("{first}{last}");

    return res.parse::<i32>().unwrap();
}

#[test]
fn should_extract_from_word() {
    let value = "two1nine".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 29);
}

#[test]
fn should_extract_from_word_n_normal_digit() {
    let value = "zoneight234".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 14);
}

#[test]
fn should_extract_from_word_n_normal_digit_1() {
    let value = "xtwone3four".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 24);
}

#[test]
fn should_extract_from_word_n_normal_digit_2() {
    let value = "3zoneight234".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 34);
}

#[test]
fn should_extract_from_word_n_normal_digit_3() {
    let value = "fiveeight792eightqskstrftdpccsrgskrhc".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 58);
}

#[test]
fn should_extract_from_word_n_normal_digit_4() {
    let value = "26fmrrhhpthree6b".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 26);
}

#[test]
fn should_extract_from_word_n_normal_digit_5() {
    let value = "4nineeightseven2".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 42);
}

#[test]
fn should_extract_from_word_n_normal_digit_6() {
    let value = "7pqrstsixteen".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 76);
}

#[test]
fn should_extract_from_word_n_normal_digit_7() {
    let value = "threenine89zlmh9fourff".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 34);
}

#[test]
fn should_extract_from_word_n_normal_digit_8() {
    let value = "m4".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 44);
}

#[test]
fn should_extract_from_word_n_normal_digit_9() {
    let value = "686135".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 65);
}

#[test]
fn should_extract_from_word_n_normal_digit_10() {
    let value = "6twone".to_string();
    let result = extract_numbers_two(value, None, true);
    assert_eq!(result, 61);
}
