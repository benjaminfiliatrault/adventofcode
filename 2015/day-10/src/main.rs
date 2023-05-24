use std::usize;

use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let mut input = ip;

                for _ in 1..=40 {
                    input = dumb_exercise(input);
                }

                println!("{:?}", input.len());
            }
        }
    }


}



fn dumb_exercise(input: String) -> String {

    let mut iter = input.chars().peekable();

    let mut buffer: Vec<char> = Vec::new();

    let mut result = String::new();

    while let Some(char) = iter.next() {
        buffer.push(char);

        if let Some(next) = iter.peek() { 
            if *next != char {
                result.extend(buffer.len().to_string().chars());
                result.push(char);
                buffer.clear();
            }

        }

        if iter.peek().is_none() {
            result.extend(buffer.len().to_string().chars());
            result.push(char);
        }
    }

    return result;
}



#[test]
fn should_return_3112() {
    let input = "1112";

    let result = dumb_exercise(input.to_string());

    assert_eq!(result, "3112");
}

#[test]
fn should_return_51() {
    let input = "11111";

    let result = dumb_exercise(input.to_string());

    assert_eq!(result, "51");
}

#[test]
fn should_return_something() {
    let input = "3113322113";

    let result = dumb_exercise(input.to_string());

    assert_eq!(result, "132123222113");
}
