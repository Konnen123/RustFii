use std::{fs, io};
fn main() {
    match ROT13() {
        Ok(cypher) => println!("Your encrypted data is: {}", cypher),
        Err(error) => println!("Error: {}", error),
    }
}

fn ROT13() -> Result<String, io::Error> {
    let data: String = fs::read_to_string("src/textToCypher.txt")?;

    let mut encrypted_data: String = String::from("");

    for ch in data.chars() {
        if !ch.is_ascii_alphabetic() {
            println!("We encountered an non ascii character: {}", ch);
            panic!();
        }

        let mut number_letter: u8 = ch as u8;
        number_letter -= 13;

        if ch.is_lowercase() {
            if number_letter < 97 {
                number_letter = 122 - (97 - number_letter) + 1;
            }
        } else {
            if number_letter < 65 {
                number_letter = 90 - (65 - number_letter) + 1;
            }
        }
        encrypted_data.push(number_letter as char);
    }
    Ok(encrypted_data)
}
