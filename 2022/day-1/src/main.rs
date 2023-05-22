use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let mut vector: Vec<u32> = Vec::new();
    let mut total = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let number = ip.parse::<u32>();
                
                if number.is_err() {
                    vector.push(total);
                    total = 0;
                    continue;
                }
        
                if number.is_ok() {
                    total += number.unwrap();
                }
            }
        }
    }
    
    vector.sort();

    print!("{}", vector.last().unwrap());

    return;
}
