#[derive(Debug)]
enum ErrorHandler {
    CharNotAscii,
    CharNotDigit,
    CharNotBase16,
    CharNotLetter,
    CharNotPrintable,
}
fn main() {
    let lowercase = 'a';
    match to_uppercase(lowercase) {
        Ok(input) => println!("{} uppercase is {}", lowercase, input),
        Err(input) => print_error(input),
    }
    let uppercase = 'Z';
    match to_lowercase(uppercase) {
        Ok(input) => println!("{} lowercase is {}", uppercase, input),
        Err(input) => print_error(input),
    }
    let not_printable = '\0';
    match print_char(not_printable) {
        Ok(input) => println!("{}", input),
        Err(input) => print_error(input),
    }
    let digit = '❤';
    match char_to_number(digit) {
        Ok(input) => println!("{}", input),
        Err(input) => print_error(input),
    }
    let hex_digit = 'F';
    match char_to_number_hex(hex_digit) {
        Ok(input) => println!("{}", input),
        Err(input) => print_error(input),
    }
}

fn to_uppercase(input: char) -> Result<char, ErrorHandler> {
    if !input.is_ascii_lowercase() && !input.is_ascii_uppercase() {
        return Err(ErrorHandler::CharNotLetter);
    }
    Ok(input.to_ascii_uppercase())
}
fn to_lowercase(input: char) -> Result<char, ErrorHandler> {
    if !input.is_ascii_lowercase() && !input.is_ascii_uppercase() {
        return Err(ErrorHandler::CharNotLetter);
    }
    Ok(input.to_ascii_lowercase())
}
fn print_char(input: char) -> Result<char, ErrorHandler> {
    if input.is_control() {
        return Err(ErrorHandler::CharNotPrintable);
    }
    Ok(input)
}
fn char_to_number(input: char) -> Result<u32, ErrorHandler> {
    if !input.is_ascii() {
        return Err(ErrorHandler::CharNotAscii);
    }
    if !input.is_ascii_digit() {
        return Err(ErrorHandler::CharNotDigit);
    }
    Ok(input.to_digit(10).unwrap())
}
fn char_to_number_hex(input: char) -> Result<u32, ErrorHandler> {
    if !input.is_ascii() {
        return Err(ErrorHandler::CharNotAscii);
    }
    if !input.is_digit(16) {
        return Err(ErrorHandler::CharNotBase16);
    }
    Ok(input.to_digit(16).unwrap())
}
fn print_error(input: ErrorHandler) {
    match input {
        ErrorHandler::CharNotAscii => println!("Character is not ascii"),
        ErrorHandler::CharNotDigit => println!("Character is not digit"),
        ErrorHandler::CharNotBase16 => println!("Character is not hexdecimal"),
        ErrorHandler::CharNotLetter => println!("Character is not a letter"),
        ErrorHandler::CharNotPrintable => println!("Character is not printable"),
    }
}
