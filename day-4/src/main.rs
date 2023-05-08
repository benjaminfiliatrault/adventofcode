use std::{collections::HashSet};

use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let mut total = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                
               let (block_1, block_2) = ip.split_once(',').unwrap(); 
               
               let (block_1_start, block_1_end) = block_1.split_once('-').unwrap();     
               let (block_2_start, block_2_end) = block_2.split_once('-').unwrap();     
   
                let block_1_start_int = block_1_start.parse::<usize>().unwrap();
                let block_1_end_int = block_1_end.parse::<usize>().unwrap();
                
                let block_2_start_int = block_2_start.parse::<usize>().unwrap();
                let block_2_end_int = block_2_end.parse::<usize>().unwrap();


                let hash_1: HashSet<usize> = (block_1_start_int..=block_1_end_int).collect();
                let hash_2: HashSet<usize> = (block_2_start_int..=block_2_end_int).collect();


                let mut diff_a = hash_1.difference(&hash_2).peekable();
                let mut diff_b = hash_2.difference(&hash_1).peekable();

                
                if diff_a.next().is_none() || diff_b.next().is_none() {
                    total += 1;
                }


            }
        }
    }

    println!("First question: {:?}", total);

    total = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                
               let (block_1, block_2) = ip.split_once(',').unwrap(); 
               
               let (block_1_start, block_1_end) = block_1.split_once('-').unwrap();     
               let (block_2_start, block_2_end) = block_2.split_once('-').unwrap();     
   
                let block_1_start_int = block_1_start.parse::<usize>().unwrap();
                let block_1_end_int = block_1_end.parse::<usize>().unwrap();
                
                let block_2_start_int = block_2_start.parse::<usize>().unwrap();
                let block_2_end_int = block_2_end.parse::<usize>().unwrap();


                let hash_1: HashSet<usize> = (block_1_start_int..=block_1_end_int).collect();
                let hash_2: HashSet<usize> = (block_2_start_int..=block_2_end_int).collect();


                let diff_a = hash_1.difference(&hash_2);
                let diff_b = hash_2.difference(&hash_1);

            
                if hash_1.len() != diff_a.count() || hash_2.len() != diff_b.count() {
                    total += 1;
                }

            }
        }
    }

    println!("Bonus: {:?}", total)

}
