use std::collections::HashMap;

use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

//         [F] [Q]         [Q]        
// [B]     [Q] [V] [D]     [S]        
// [S] [P] [T] [R] [M]     [D]        
// [J] [V] [W] [M] [F]     [J]     [J]
// [Z] [G] [S] [W] [N] [D] [R]     [T]
// [V] [M] [B] [G] [S] [C] [T] [V] [S]
// [D] [S] [L] [J] [L] [G] [G] [F] [R]
// [G] [Z] [C] [H] [C] [R] [H] [P] [D]
//  1   2   3   4   5   6   7   8   9 


fn main() {
    if let Ok(lines) = read_lines(FILE_PATH) {
        let mut queue_completed = false;
        let mut queue: HashMap<usize, Vec<char>> = HashMap::new();


        for line in lines {
            if let Ok(ip) = line {

                // Create the queue without Regex because no...
                if !queue_completed {
                    let (q, c) = build_queue(ip.clone(), &mut queue);
                    queue = q.to_owned();
                    queue_completed = c;
                    continue;
                }

                if ip == "" { continue };

                let mut values = ip.split_whitespace();
                let nb_crates = values.nth(1).unwrap().parse::<usize>().unwrap();
                let move_from = values.nth(1).unwrap().parse::<usize>().unwrap();
                let move_to = values.nth(1).unwrap().parse::<usize>().unwrap();
                let mut current: Vec<char> = Vec::new();

                if let Some(x) = queue.get_mut(&move_from) {
                    let index = x.len() - nb_crates;
                    let mut crates: Vec<char> = x.drain(index..).collect();
                    crates.reverse();
                    current.append(&mut crates);
                }


                if let Some(y) = queue.get_mut(&move_to) {
                    y.append(&mut current);
                }
            }
        }
        pretty_print("First answer", queue);
    }



    if let Ok(lines) = read_lines(FILE_PATH) {
        let mut queue_completed = false;
        let mut queue: HashMap<usize, Vec<char>> = HashMap::new();

        for line in lines {
            if let Ok(ip) = line {

                // Create the queue without Regex because no...
                if !queue_completed {
                    let (q, c) = build_queue(ip.clone(), &mut queue);
                    queue = q.to_owned();
                    queue_completed = c;
                    continue;
                }

                if ip == "" { continue };

                let mut values = ip.split_whitespace();
                let nb_crates = values.nth(1).unwrap().parse::<usize>().unwrap();
                let move_from = values.nth(1).unwrap().parse::<usize>().unwrap();
                let move_to = values.nth(1).unwrap().parse::<usize>().unwrap();

                let mut current: Vec<char> = Vec::new();

                if let Some(x) = queue.get_mut(&move_from) {
                    let index = x.len() - nb_crates;
                    let mut crates: Vec<char> = x.drain(index..).collect();
                    current.append(&mut crates);
                }

                if let Some(y) = queue.get_mut(&move_to) {
                    y.append(&mut current);
                }
            }
        }
        pretty_print("Bonus", queue);
    }

}


fn pretty_print(message: &str, queue: HashMap<usize, Vec<char>>) {
    let mut letters = Vec::new();

    for x in 1..=9 {
        let stack = queue.get(&x).expect("Cannot get the value");
        let last_letter = stack.last().expect("Cannot get the last value");
        letters.push(last_letter);
    }

    let letters_message = String::from_iter(letters);

    println!("{:}: {:}", message, letters_message);
}


fn build_queue(ip: String, mut queue: &mut HashMap<usize, Vec<char>>) -> (&HashMap<usize, Vec<char>>, bool) {
    let mut line_char = ip.chars().enumerate();

    while let Some((idx, char)) = line_char.next() {
        match char {
            'A'..='Z' => {
                if let Some(v) = queue.get_mut(&idx) {
                    v.insert(0, char);
                    continue;
                }

                queue.insert(idx, Vec::from([char]));
            },
            '1'..='9' => {
                let values = queue.get(&idx).unwrap().clone();
                let stack_number = char.to_digit(10).expect("Cannot convert char to digit");
                queue.remove(&idx);
                queue.insert(stack_number as usize, values);

                if stack_number == 9 {
                    return (queue, true);
                }
            }
            _ => {} 
        }

    }

    return (queue, false);
}
