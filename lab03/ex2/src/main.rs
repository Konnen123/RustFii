fn main() {
    println!("{}", check_mul(u32::MAX, 30));
    println!("{}", check_add(u32::MAX, 30));
}
fn check_mul(x: u32, y: u32) -> u32 {
    let mul: u64 = (x * y) as u64;
    if mul > u32::MAX as u64 {
        panic!("Out of range");
    }
    mul as u32
}
fn check_add(x: u32, y: u32) -> u32 {
    let add: u64 = (x + y) as u64;
    if add > u32::MAX as u64 {
        panic!("Out of range");
    }
    add as u32
}
