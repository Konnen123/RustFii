use std::{fs, io};
use std::collections::HashMap;
#[derive(Debug)]
enum ROT13_Errors 
{
    NonAscii,
    CantWriteToFile,
}
fn main() ->Result<(), io::Error>
{
    let mut data: String = fs::read_to_string("src/input.txt")?;
    match ROT13(&data)
    {
        Ok(()) =>println!(),
        Err(error)=>println!("{:?}",error),
    }
    Ok(())
}

fn ROT13(data :  &str) -> Result<(), ROT13_Errors> {

    let rot13_map = create_hashmap();
    let mut cypher = String::from("");
    for c in data.chars()
    {
        if !c.is_ascii()
        {
            return Err(ROT13_Errors::NonAscii)
        }
    
        match rot13_map.get(&c)
        {
            Some(&ch) =>cypher.push(ch),
            None =>cypher.push(c),
        }

    }
    match fs::write("src/output.txt", cypher)
    {
        Ok(ok)=>return Ok(()),
        Err(error)=>return Err(ROT13_Errors::CantWriteToFile),
    }
  
    Ok(())
}
fn create_hashmap() -> HashMap<char,char>
{
    let mut rot13_map: HashMap<char, char> = HashMap::new();
    rot13_map.insert('a', 'n');
    rot13_map.insert('b', 'o');
    rot13_map.insert('c', 'p');
    rot13_map.insert('d', 'q');
    rot13_map.insert('e', 'r');
    rot13_map.insert('f', 's');
    rot13_map.insert('g', 't');
    rot13_map.insert('h', 'u');
    rot13_map.insert('i', 'v');
    rot13_map.insert('j', 'w');
    rot13_map.insert('k', 'x');
    rot13_map.insert('l', 'y');
    rot13_map.insert('m', 'z');
    rot13_map.insert('n', 'a');
    rot13_map.insert('o', 'b');
    rot13_map.insert('p', 'c');
    rot13_map.insert('q', 'd');
    rot13_map.insert('r', 'e');
    rot13_map.insert('s', 'f');
    rot13_map.insert('t', 'g');
    rot13_map.insert('u', 'h');
    rot13_map.insert('v', 'i');
    rot13_map.insert('w', 'j');
    rot13_map.insert('x', 'k');
    rot13_map.insert('y', 'l');
    rot13_map.insert('z', 'm');

    rot13_map.insert('A', 'N');
    rot13_map.insert('B', 'O');
    rot13_map.insert('C', 'P');
    rot13_map.insert('D', 'Q');
    rot13_map.insert('E', 'R');
    rot13_map.insert('F', 'S');
    rot13_map.insert('G', 'T');
    rot13_map.insert('H', 'U');
    rot13_map.insert('I', 'V');
    rot13_map.insert('J', 'W');
    rot13_map.insert('K', 'X');
    rot13_map.insert('L', 'Y');
    rot13_map.insert('M', 'Z');
    rot13_map.insert('N', 'A');
    rot13_map.insert('O', 'B');
    rot13_map.insert('P', 'C');
    rot13_map.insert('Q', 'D');
    rot13_map.insert('R', 'E');
    rot13_map.insert('S', 'F');
    rot13_map.insert('T', 'G');
    rot13_map.insert('U', 'H');
    rot13_map.insert('V', 'I');
    rot13_map.insert('W', 'J');
    rot13_map.insert('X', 'K');
    rot13_map.insert('Y', 'L');
    rot13_map.insert('Z', 'M');
    rot13_map
}