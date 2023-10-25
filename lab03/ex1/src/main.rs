fn main() {
    let mut number: u16 = 1;
    loop {
        let next_prime = next_prime(number);
        match next_prime {
            Some(x) => println!("next prime number for {} is {}", number, x),
            None => {
                println!("we reached the final number {}", number);
                break;
            }
        }
        number += 1;
    }
}
fn next_prime(x: u16) -> Option<u16> {
    let mut next_prime: u32 = x as u32;
    if next_prime < 2 {
        return Some(2);
    }

    let mut ok: bool;
    loop {
        ok = true;
        for i in 2..next_prime {
            if next_prime % i == 0 {
                ok = false;
                break;
            }
        }
        if ok {
            break;
        }
        next_prime += 1;
    }
    if next_prime > u16::MAX as u32 {
        return None;
    }
    Some(next_prime as u16)
}
