use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let mut message_position = 0;
                let mut buffer: Vec<char> = vec![];
                
                let mut iter = ip.chars().enumerate();

                while let Some((idx, char)) = iter.next() {
                    let mut buffer_iter = buffer.iter();
                    if let Some(found_char) = buffer_iter.rposition(|c| c == &char) {
                        buffer.drain(..=found_char);
                    }

                    buffer.push(char);

                    if buffer.len() == 14 {
                        message_position = idx + 1;
                        break;
                    }
                }

                println!("Position: {:?}", message_position);
            }
        }
    }
}


// [1,4,3,4,5,6,7,8]
//           ^
//
// Buffer: 
// [3,4,1]
