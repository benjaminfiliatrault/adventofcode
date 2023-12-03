use std::usize;

use read_line::read_lines;

const FILE_PATH: &str = "./src/input.txt";

fn main() {
    
    let mut grid_trees: Vec<Vec<u32>> = vec![];

    if let Ok(lines) = read_lines(FILE_PATH) {
        for line in lines {
            if let Ok(ip) = line {
                let mut iter = ip.chars();

                let mut columns = Vec::new();

                while let Some(c) = iter.next() {
                    let number = c.to_digit(10).expect("Cannot parse");
                    columns.push(number);
                }

                grid_trees.push(columns);
            }
        }
    }

    let mut visible_trees: usize = 0;

    for (row_idx, rows) in grid_trees.iter().enumerate() {
        if row_idx == 0 { continue; }
        if row_idx == grid_trees.len() - 1 { continue; }
        
        for (col_idx, current_number) in rows.iter().enumerate() {
            if col_idx == 0 { continue; }
            if col_idx == rows.len() - 1 { continue; }

            
            let row_lookup = grid_trees[row_idx].iter().enumerate();  

            let mut is_left_visible = true;
            let mut is_right_visible = true;


            // Row lookup
            for (idx, number) in row_lookup {
                if idx == col_idx { continue; }

                if number >= current_number && idx < col_idx {
                    is_left_visible = false;
                }

                if number >= current_number && idx > col_idx {
                    is_right_visible = false;
                    break;
                }
            }

            let col_lookup = grid_trees.iter().enumerate();

            let mut is_top_visible = true;
            let mut is_bottom_visible = true;

            // Column lookup
            for (idx, col) in col_lookup {
                let number = col[col_idx];

                if idx == row_idx { continue; }

                if number >= *current_number && idx < row_idx {
                    is_top_visible = false;
                }

                if number >= *current_number && idx > row_idx {
                    is_bottom_visible = false;
                    break;
                }
            }


            if is_left_visible || is_right_visible || is_top_visible || is_bottom_visible {
                visible_trees += 1;
            }
        }

    }

    let rows_count = grid_trees.len();
    let col_count = grid_trees[0].len();

    // Add twice the rows & columns because the exterior trees 
    // are all visible
   visible_trees += (rows_count * 2) + ((col_count - 2) * 2); 

    println!("{:}", visible_trees);




    // BONUS DOWN THERE

    let mut scenic_scores = Vec::new();

    for (row_idx, rows) in grid_trees.iter().enumerate() {
        for (col_idx, current_number) in rows.iter().enumerate() {
            let mut row_lookup = grid_trees[row_idx].iter().enumerate().peekable();  

            let mut is_left_visible = true;

            let mut left_amount_visible = 0;
            let mut right_amount_visible = 0;

            // Row lookup
            while let Some((idx, number)) = row_lookup.next() {
                if idx == col_idx {
                    if is_left_visible {
                        left_amount_visible = idx;
                    }
                    continue;
                }

                if number >= current_number && idx < col_idx {
                    left_amount_visible = col_idx - idx;
                    is_left_visible = false;
                }

                if number >= current_number && idx > col_idx {
                    right_amount_visible = idx - col_idx;
                    break;
                }

                if row_lookup.peek().is_none() {
                    right_amount_visible = idx - col_idx;
                }
            }

            let mut col_lookup = grid_trees.iter().enumerate().peekable();
            
            let mut is_top_visible = true;
            let mut top_amount_visible = 0;
            let mut bottom_amount_visible = 0;

            // Column lookup
            while let Some((idx, col)) = col_lookup.next() {
                if idx == row_idx {
                    if is_top_visible {
                        top_amount_visible = idx;
                    }
                    continue;
                }

                let number = col[col_idx];

                if number >= *current_number && idx < row_idx {
                    top_amount_visible = row_idx - idx;   
                    is_top_visible = false;
                }

                if number >= *current_number && idx > row_idx {
                    bottom_amount_visible = idx - row_idx;
                    break;
                }

                if col_lookup.peek().is_none() {
                    bottom_amount_visible = idx - row_idx;
                }
            }

            let scenic_score = left_amount_visible * right_amount_visible * top_amount_visible * bottom_amount_visible;

            scenic_scores.push(scenic_score);
        }

    }


    let highest_scene = scenic_scores.iter().max().expect("Cannot find the max value");

    println!("{:?}", highest_scene);

}
