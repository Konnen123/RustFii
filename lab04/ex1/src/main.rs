use std::{fs, io};
fn main() {
    match max_length_line() {
        Ok(x) => println!("The max line by character length is: {}", x),
        Err(x) => println!("Error: {}", x),
    }
    match max_bytes_line() {
        Ok(x) => println!("The max line by bytes length is: {}", x),
        Err(x) => println!("Error: {}", x),
    }
}

fn max_length_line() -> Result<String, io::Error> {
    let s = fs::read_to_string("src/file.txt")?;

    let all_lines = s.lines();

    let mut max_line: &str = "";
    for line in all_lines {
        if line.chars().count() > max_line.chars().count() {
            max_line = line;
        }
    }
    Ok(max_line.to_string())
}
fn max_bytes_line() -> Result<String, io::Error> {
    let s = fs::read_to_string("src/file.txt")?;

    let all_lines = s.lines();

    let mut max_line: &str = "";
    for line in all_lines {
        if line.len() > max_line.len() {
            max_line = line;
        }
    }

    Ok(max_line.to_string())
}
