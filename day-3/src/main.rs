use std::collections::HashSet;

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
                    total = total + value as u32 + 1; // Plus 1 because the Vec is zero-based
                }
            }
        }
    }

    println!("First total: {:?}", total);

    total = 0; 
    let mut string_groups: Vec<Vec<String>> = Vec::new();
    let mut tmp_group = Vec::new();

     if let Ok(lines) = read_lines(FILE_PATH) {
        let mut lines_list = lines.peekable();

    
        let mut idx = 0;

        while let Some(line) = lines_list.next() {
             if let Ok(ip) = line {
                if idx % 3 == 0 {
                    if idx != 0 {
                        string_groups.push(tmp_group.clone());
                    }

                    tmp_group.clear();
                }
                
                tmp_group.push(ip);


                if idx % 3 != 0 && lines_list.peek().is_none() {
                    string_groups.push(tmp_group.clone());
                }
            } 
            idx += 1;
        }

    }


     for group in string_groups {
         let found_char = find_matching_char_group(group);
         let priority_value = priorities.iter().position(|&c| c == found_char);

        if let Some(value) = priority_value {
            total = total + value as u32 + 1; // Plus 1 because the Vec is zero-based
        }
     }

    println!("Bonus total: {:}", total);

}


// Expect the string to only have 1 repeating char 
fn find_matching_char(value: String) -> char {
    let (block_1, block_2) = value.split_at((value.len() + 1) / 2);
    
    let set1: HashSet<char> = HashSet::from_iter(block_1.chars());
    let set2: HashSet<char> = HashSet::from_iter(block_2.chars());

    let found_char = set1.intersection(&set2).next().unwrap();

    return found_char.to_owned();
}

fn find_matching_char_group(value: Vec<String>) -> char {
    let line1 = value[0].clone();
    let line2 = value[1].clone();
    let line3 = value[2].clone();
    
    let set1: HashSet<char> = HashSet::from_iter(line1.chars());
    let set2: HashSet<char> = HashSet::from_iter(line2.chars());
    let set3: HashSet<char> = HashSet::from_iter(line3.chars());

    let a = set1.intersection(&set2).map(|c| c.to_owned());

    let b: HashSet<char> = HashSet::from_iter(a);
    

   let mut found_char = set3.intersection(&b); 

   return found_char.next().unwrap().to_owned();
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

#[test]
fn should_return_repeating_char_in_string_vec() {
    let s1 =String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
    let s2 = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    let s3 = String::from("PmmdzqPrVvPwwTWBwg");

    let string_vect = vec![s1, s2, s3];

    let result = find_matching_char_group(string_vect);

    assert_eq!(result, 'r');
}

#[test]
fn should_return_repeating_char_in_string_veci_2() {
    let s1 =String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
    let s2 = String::from("ttgJtRGJQctTZtZT");
    let s3 = String::from("CrZsJsPPZsGzwwsLwLmpwMDw");

    let string_vect = vec![s1, s2, s3];

    let result = find_matching_char_group(string_vect);

    assert_eq!(result, 'Z');
}

#[test]
fn should_return_repeating_char_in_string_veci_3() {
    let s1 =String::from("LHLRlCCvCLVgHPfCHtVjBGrBDNzWFBsBGBfscGsD");
    let s2 = String::from("nQwbnwwpbrJBrNWB");
    let s3 = String::from("hmnSdSdQpTpdnlPdvddPNglLjH");

    let string_vect = vec![s1, s2, s3];

    let result = find_matching_char_group(string_vect);

    assert_eq!(result, 'N');
}
