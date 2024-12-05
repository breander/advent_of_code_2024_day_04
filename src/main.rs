use std::env;
use std::fs;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();

    // check for filename
    if args.len() < 2 {
        println!("No file name specified!");
        return;
    }

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();

    // setup grid and counts
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut line_count = 0;
    let mut col_count = 0;
    let mut part1_count = 0;

    // load text into 2d vector
    for line in lines {
        let characters: Vec<char> = line.chars().collect(); 
        col_count = characters.len();
        grid.push(characters);
        line_count += 1;
    }

    let mut right = 0;
    let mut right_down = 0;
    let mut down = 0;
    let mut down_left = 0;
    let mut left = 0;
    let mut left_up = 0;
    let mut up = 0;
    let mut up_right = 0;

    // iterate over grid to check for XMAS
    for row in 0..line_count {
        for col in 0..col_count {
            // check right
            if (col + 3) < col_count {
                if grid[row][col] == 'X' && grid[row][col+1] == 'M' && grid[row][col+2] == 'A' && grid[row][col+3] == 'S' {
                    part1_count += 1;
                    right += 1;
                }
            }

            // check right + down
            if (col + 3) < col_count && (row + 3) < line_count {
                if grid[row][col] == 'X' && grid[row+1][col+1] == 'M' && grid[row+2][col+2] == 'A' && grid[row+3][col+3] == 'S' {
                    part1_count += 1;
                    right_down += 1;
                }
            }

            // check down
            if (row + 3) < line_count {
                if grid[row][col] == 'X' && grid[row+1][col] == 'M' && grid[row+2][col] == 'A' && grid[row+3][col] == 'S' {
                    part1_count += 1;
                    down += 1;
                }
            }

            // check down + left
            if (row + 3) < line_count && (col as i32 - 3) >= 0 {
                if grid[row][col] == 'X' && grid[row+1][col-1] == 'M' && grid[row+2][col-2] == 'A' && grid[row+3][col-3] == 'S' {
                    part1_count += 1;
                    down_left += 1;
                }
            }

            // check left
            if (col as i32 - 3) >= 0 {
                if grid[row][col] == 'X' && grid[row][col-1] == 'M' && grid[row][col-2] == 'A' && grid[row][col-3] == 'S' {
                    part1_count += 1;
                    left += 1;
                }
            }

            // check left + up
            if (col as i32 - 3) >= 0 && (row as i32 - 3) >= 0 {
                if grid[row][col] == 'X' && grid[row-1][col-1] == 'M' && grid[row-2][col-2] == 'A' && grid[row-3][col-3] == 'S' {
                    part1_count += 1;
                    left_up += 1;
                }
            }

            // check up
            if (row as i32 - 3) >= 0 {
                if grid[row][col] == 'X' && grid[row-1][col] == 'M' && grid[row-2][col] == 'A' && grid[row-3][col] == 'S' {
                    part1_count += 1;
                    up += 1;
                }
            }

            // check up + right
            if (row as i32 - 3) >= 0 && (col + 3) < col_count {
                if grid[row][col] == 'X' && grid[row-1][col+1] == 'M' && grid[row-2][col+2] == 'A' && grid[row-3][col+3] == 'S' {
                    part1_count += 1;
                    up_right += 1;
                }
            }
        }
    }

    println!("right: {right}");
    println!("right_down: {right_down}");
    println!("down: {down}");
    println!("down_left: {down_left}");
    println!("left: {left}");
    println!("left_up: {left_up}");
    println!("up: {up}");
    println!("up_right: {up_right}");
    println!("part 1 XMAS count: {part1_count}");

    // Part - 2
    let mut part2_count = 0;

    // iterate over grid to check for XMAS
    for row in 0..line_count {
        for col in 0..col_count {
            if grid[row][col] == 'A' {
                let mut two_count = 0;

                if (col + 1) < col_count && (row + 1) < line_count && (col as i32 - 1) >= 0 && (row as i32 - 1) >= 0  {
                    // check down + right
                    if grid[row-1][col-1] == 'M' && grid[row+1][col+1] == 'S' {
                        two_count += 1;
                    }

                    // check down + left
                    if grid[row-1][col+1] == 'M' && grid[row+1][col-1] == 'S' {
                        two_count += 1;
                    }

                    // check up + left
                    if grid[row+1][col+1] == 'M' && grid[row-1][col-1] == 'S' {
                        two_count += 1;
                    }

                    // check up + right
                    if grid[row+1][col-1] == 'M' && grid[row-1][col+1] == 'S' {
                        two_count += 1;
                    }

                    if two_count == 2 {
                        part2_count += 1;
                    }
                }
            }
        }
    }

    println!("part 2 X-MAS count: {part2_count}");
}

