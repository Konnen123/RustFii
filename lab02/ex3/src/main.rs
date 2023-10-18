fn main() {
    let mut s = String::from("");
    add_space(&mut s, 42);
    add_str(&mut s, "I 💚\n");
    add_space(&mut s, 42);
    add_str(&mut s, "RUST.\n\n");
    add_space(&mut s, 6);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 6);
    add_str(&mut s, "lastest");
    add_space(&mut s, 10);
    add_str(&mut s, "is\n");
    add_space(&mut s, 11);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 10);
    add_str(&mut s, "version");
    add_space(&mut s, 5);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");

    println!("{s}");
}
fn add_space(str: &mut String, spaces: u32) {
    for i in 0..spaces {
        str.push(' ');
    }
}
fn add_str(str: &mut String, s: &str) {
    str.push_str(s);
}
fn add_integer(str: &mut String, number: i32) {
    let number_as_string = number.to_string();
    let mut count = 0;
    for c in number_as_string.chars() {
        if count % 3 == 0 && count != 0 {
            str.push('_');
        }
        str.push(c);
        count += 1;
    }
}
fn add_float(str: &mut String, number: f32) {
    str.push_str(number.to_string().as_str());
}
