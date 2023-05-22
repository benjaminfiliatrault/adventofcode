use read_line::read_lines;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOST: u32 = 0;

const X: u32 = 1;
const Y: u32 = 2;
const Z: u32 = 3;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    let mut total: u32 = 0;

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let chars = ip.split(' ').enumerate();
                let mut cloned_chars = chars.clone();

                for (idx, c) in chars {
                    let next_char = cloned_chars.nth(idx + 1);

                    if next_char == None { continue; }

                    let char = next_char.unwrap();

                    // Rock (A) & Rock (X)
                    if c == "A" && char.1 == "X" {
                        total += DRAW + X;
                    }
                    // Rock (A) & Paper (Y)
                    if c == "A" && char.1 == "Y" {
                        total += WIN + Y;
                    }
                    // Rock (A) & Scissors (Z)
                    if c == "A" && char.1 == "Z" {
                        total += LOST + Z;
                    }

                    // Paper (B) & Rock (X)
                    if c == "B" && char.1 == "X" {
                        total += LOST + X;
                    }

                    // Paper (B) & Paper (Y)
                    if c == "B" && char.1 == "Y" {
                        total += DRAW + Y;
                    }

                    // Paper (B) & Scissors (Z)
                    if c == "B" && char.1 == "Z" {
                        total += WIN + Z;
                    }

                    // Scissors (C) & Rock (X)
                    if c == "C" && char.1 == "X" {
                        total += WIN + X;
                    }
                    
                    // Scissors (C) & Paper (Y)
                    if c == "C" && char.1 == "Y" {
                        total += LOST + Y;
                    }

                    // Scissors (C) & Scissors (Z)
                    if c == "C" && char.1 == "Z" {
                        total += DRAW + Z;
                    }
                }
            }
        }
    }

    print!("{}", total);
}
