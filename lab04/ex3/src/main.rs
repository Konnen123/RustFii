use std::collections::HashMap;
use std::{fs, io};
fn main() {
    match replace_abreviations() {
        Ok(output) => println!("{}", output),
        Err(error) => println!("{}", error),
    }
}

fn replace_abreviations() -> Result<String, io::Error> {
    let mut abreviations: HashMap<&str, &str> = HashMap::new();
    abreviations.insert("pt", "pentru");
    abreviations.insert("ptr", "pentru");
    abreviations.insert("dl", "domnul");
    abreviations.insert("dna", "doamna");

    let abreviated_text = fs::read_to_string("src/abreviatedText.txt")?;

    let sliced_text: Vec<&str> = abreviated_text.split(' ').collect();
    let mut correct_text = String::from("");

    for split in sliced_text {
        if abreviations.contains_key(split) {
            correct_text.push_str(abreviations[split]);
            correct_text.push(' ');
        } else {
            correct_text.push_str(split);
            correct_text.push(' ');
        }
    }

    Ok(correct_text)
}
