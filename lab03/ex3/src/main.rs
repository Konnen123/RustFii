#[derive(Debug)]
enum ErrorHandler {
    OutOfRange,
}
fn main() {
    match check_mul(u32::MAX, 30) {
        Ok(i) => println!("The result is {}", i),
        Err(z) => println!("{:?}", z),
    }
    match check_add(30, 40) {
        Ok(i) => println!("The result is {}", i),
        Err(z) => println!("{:?}", z),
    }
}
fn check_mul(x: u32, y: u32) -> Result<u32, ErrorHandler> {
    let mul: u64 = x as u64 * y as u64;
    if mul > u32::MAX as u64 {
        return Err(ErrorHandler::OutOfRange);
    }
    Ok(mul as u32)
}
fn check_add(x: u32, y: u32) -> Result<u32, ErrorHandler> {
    let add: u64 = x as u64 + y as u64;
    if add > u32::MAX as u64 {
        return Err(ErrorHandler::OutOfRange);
    }
    Ok(add as u32)
}
