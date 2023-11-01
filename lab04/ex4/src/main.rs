use std::{collections::VecDeque, fs, io};

fn main() {
    match read_host_file() {
        Ok(info) => println!("{}", info),
        Err(error) => println!("Error: {}", error),
    }
}

fn read_host_file() -> Result<String, io::Error> {
    let host_data: String = fs::read_to_string("C:/Windows/System32/drivers/etc/hosts")?;
    let host_data_as_lines = host_data.lines();

    let mut data = String::from("");
    for line in host_data_as_lines {
        match line.chars().nth(0) {
            Some(x) => {
                if x == '#' {
                    continue;
                }
            }
            None => continue,
        }

        let vec_data: Vec<&str> = line.split(" ").collect();

        data.push_str(vec_data[1]);
        data.push_str(" => ");
        data.push_str(vec_data[0]);
        data.push('\n');
    }

    Ok(data)
}
