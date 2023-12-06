use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        match u64::from_str(&arg) {
            Ok(n) => { numbers.push(n); },
            Err(err) => { println!("error: {}", err); }
        }
    }

    if numbers.len() == 0 {
        println!("Usage: gcd NUMBER ...");
        return;
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}",
             numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        // The following is a let statement. It declares a variable named
        // "old_m" and assigns it the value of "m". The "mut" keyword means
        // that the variable is mutable, i.e. it can be changed.
        let old_m = m;
        // The following is a let statement. It declares a variable named
        // "m" and assigns it the value of "n % m". The "%" operator is the
        // modulo operator. It returns the remainder of the division of "n"
        // by "m".
        m = n % m;
        // The following is a let statement. It declares a variable named
        // "n" and assigns it the value of "old_m". The "mut" keyword means
        // that the variable is mutable, i.e. it can be changed.
        n = old_m;
    }
    // The following is a return statement. It returns the value of "n".
    n
}

// The following is a test function. It is run when the "cargo test" command
// is executed. It tests the "gcd" function.
#[test]
fn test_gcd() {
    // The following is an assert! macro. It checks that the value of the
    // "gcd" function with the arguments 14 and 15 is equal to 1. If it is
    // not, it prints the message "gcd(14, 15) != 1" and aborts the program.
    assert_eq!(gcd(14, 15), 1);
    // The following is an assert! macro. It checks that the value of the
    // "gcd" function with the arguments 2 and 4 is equal to 2. If it is
    // not, it prints the message "gcd(2, 4) != 2" and aborts the program.
    assert_eq!(gcd(2, 4), 2);
    // The following is an assert! macro. It checks that the value of the
    // "gcd" function with the arguments 9 and 6 is equal to 3. If it is
    // not, it prints the message "gcd(9, 6) != 3" and aborts the program.
    assert_eq!(gcd(9, 6), 3);
    // The following is an assert! macro. It checks that the value of the
    // "gcd" function with the arguments 3 and 3 is equal to 3. If it is
    // not, it prints the message "gcd(3, 3) != 3" and aborts the program.
    assert_eq!(gcd(3, 3), 3);
}
