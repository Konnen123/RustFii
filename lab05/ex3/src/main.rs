use std::{fmt::format, fs, io};
const ROWS: usize = 17;
const COLOUMNS: usize = 17;
type Matrix = [[char; ROWS]; COLOUMNS];
fn main() {
    let iterations: u8 = 3;
    match live_game(iterations) {
        Ok(()) => println!("Live game finished!"),
        Err(error) => println!("Error: {}", error),
    }
}

fn live_game(iterations: u8) -> Result<(), io::Error> {
    let matrix_as_string = fs::read_to_string("src/matrixOfLife.txt")?;

    let mut default_matrix: Matrix = [[' '; ROWS]; COLOUMNS];
    let mut final_matrix: Matrix = [[' '; ROWS]; COLOUMNS];

    let mut r = 0;
    let mut c = 0;
    for alive in matrix_as_string.chars() {
        if alive.is_control() {
            continue;
        }
        default_matrix[r][c] = alive;
        c += 1;
        if c % COLOUMNS == 0 {
            r += 1;
            c = 0;
        }
    }
    for iteration in 0..iterations {
        let mut output_file = String::from("");
        for i in 0..ROWS {
            for j in 0..COLOUMNS {
                let alive_neighbours = count_alive_neighbours(i, j, &default_matrix);
                if default_matrix[i][j] == 'x' {
                    if alive_neighbours == 2 || alive_neighbours == 3 {
                        final_matrix[i][j] = 'x';
                    }
                } else {
                    if alive_neighbours == 3 {
                        final_matrix[i][j] = 'x';
                    }
                }

                output_file.push(final_matrix[i][j]);
            }
            output_file.push('\n');
        }
        let path = format!("iteration{}.txt", iteration);
        match fs::write(path, output_file) {
            Ok(()) => println!("Iteration {} succeeded", iteration),
            Err(error) => println!("Error at iteration {} due to: {}", iteration, error),
        }
        default_matrix = final_matrix.clone();

        //clear the matrix
        for i in 0..ROWS {
            for j in 0..COLOUMNS {
                final_matrix[i][j] = ' ';
            }
        }
    }

    Ok(())
}

fn count_alive_neighbours(i: usize, j: usize, default_matrix: &Matrix) -> u8 {
    let mut count: u8 = 0;
    //above cell cases
    if i > 0 && j > 0 && default_matrix[i - 1][j - 1] == 'x' {
        count += 1;
    }
    if i > 0 && default_matrix[i - 1][j] == 'x' {
        count += 1;
    }
    if i > 0 && j < COLOUMNS - 1 && default_matrix[i - 1][j + 1] == 'x' {
        count += 1;
    }
    //middle cell cases

    if j > 0 && default_matrix[i][j - 1] == 'x' {
        count += 1;
    }
    if j < COLOUMNS - 1 && default_matrix[i][j + 1] == 'x' {
        count += 1;
    }

    //bottom cell cases
    if i < ROWS - 1 && j > 0 && default_matrix[i + 1][j - 1] == 'x' {
        count += 1;
    }
    if i < ROWS - 1 && default_matrix[i + 1][j] == 'x' {
        count += 1;
    }
    if i < ROWS - 1 && j < COLOUMNS - 1 && default_matrix[i + 1][j + 1] == 'x' {
        count += 1;
    }
    count
}
