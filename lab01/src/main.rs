fn main() {
    for i in 0..=100 {
        printPrimeNumber(i);
    }

    for i in 0..=100 {
        for j in 0..=100 {
            checkCoPrimeNumbers(i, j);
        }
    }
    print99Bottles();
}

#[allow(non_snake_case)]
fn printPrimeNumber(number: u32) {
    if number < 2 {
        return;
    } else if number % 2 == 0 && number != 2 {
        return;
    } else {
        for i in 3..((number as f64).sqrt() as u32) {
            if number % i == 0 {
                return;
            }
        }
    }

    println!("{}", number);
}
#[allow(non_snake_case)]
fn checkCoPrimeNumbers(first: u32, second: u32) ->bool {
    if first < 2 || second < 2 {
        return false;
    }
    let min = first.min(second);
    let max = first.max(second);

    for i in 2..min {
        if max % i == 0 && min % i == 0 {
            return false;
        }
    }
    println!("{} and {} are coprime", min, max);
    true
}
#[allow(non_snake_case)]
fn print99Bottles() {
    let mut numberOfBottles: u32 = 99;

    while numberOfBottles != 1 {
        println!(
            "{} bottles of beer on the wall, {} bottles of beer.",
            numberOfBottles, numberOfBottles
        );
        numberOfBottles -= 1;
        println!(
            "Take one down and pass it around, {} bottles of beer on the wall.",
            numberOfBottles
        );
        println!("");
    }
    println!("1 bottle of beer on the wall, 1 bottle of beer.");
    println!("Take one down and pass it around, no more bottles of beer on the wall.");
    println!("");

    println!("No more bottles of beer on the wall, no more bottles of beer.");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
}
