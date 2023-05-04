use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let mut priorities: Vec<char> = Vec::new();
    let mut total: u32 = 0;

    for x in 'a'..='z' {
        priorities.push(x);
    }

    for x in 'A'..='Z' {
        priorities.push(x);
    }

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let found_char:char  = find_matching_char(ip);
                let priority_value = priorities.iter().position(|&c| c == found_char);

                if let Some(value) = priority_value {
                    println!("char:{:?}  value: {:?}", found_char, value);
                    total = total + value as u32 + 1; // Plus 1 because the Vec is zero-based
                }
            }
        }
    }

    println!("{:?}", total);
}


// Expect the string to only have 1 repeating char 
fn find_matching_char(value: String) -> char {
    let (block_1, block_2) = value.split_at((value.len() + 1) / 2);

    let mut found_char = '\n';

    for char_1 in block_1.chars() {
        for char_2 in block_2.chars() {
            if char_1 == char_2 {
                found_char = char_1;
            }
        } 
    }

    return found_char;
}


#[test]
fn should_match_the_char_correctly() {
    let s = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
    let result = find_matching_char(s);
    assert_eq!(result, 'p');

    let s2 = String::from("CrZsJsPPZsGzwwsLwLmpwMDw");
    let result_2 = find_matching_char(s2);
    assert_eq!(result_2, 's');
}




