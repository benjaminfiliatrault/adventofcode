use std::collections::HashMap;

use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let mut file_system: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    let mut current_dir = String::with_capacity(128);


    // Build the tree
    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with('$') {
                    if ip.contains("cd") {
                        let x: &[_] = &['$', ' ', 'c', 'd'];
                        let folder = ip.trim_matches(x);
                        
                        // We need to go up in the tree
                        if folder == ".." {
                            current_dir = current_dir.trim_end_matches(|c| c == '/').to_string();
                            continue;
                        }

                        // We need to move down the tree
                        current_dir = current_dir.to_owned() + "/" + folder;
                        if folder == "/" {
                           current_dir = "/".to_string(); 
                        }
                        // Make sure we keep track of that directory
                        file_system.insert(current_dir.clone(), Vec::new());
                        }

                    if ip.contains("ls") {
                        // We don't really care do we?
                    }

                    continue;
                } 
               
                // If it's not a command it can only be information
                // so split once on space.
                let (start, name) = ip.split_once(' ').expect("Cannot split the line"); 
                
                // File with a defined size
                if let Ok(size) = start.parse::<usize>() {
                    if let Some(subfolders) = file_system.get_mut(&current_dir) {
                        subfolders.push((size, name.to_owned()));
                    }            
                }

                if start == "dir" {
                    if let Some(subfolders) = file_system.get_mut(&current_dir) {
                        subfolders.push((0, name.to_owned() + "/"));
                    } 
                }
    
                
            }
        }
    }

    println!("{:?}", file_system.get("/csmqbhjv").unwrap());

    // Parse the tree and calculate the total size 

}



//            /
//              /dir_1
//                  file_1 - 2
//                  file_2 - 1
//                  file_3 - 5
//                  dir_3/
//                      file_4 - 1
//              dir_3/
//                  file_5 - 8v
