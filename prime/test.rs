use std::time::{SystemTime, UNIX_EPOCH};

fn isPrime(n: i32) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n == 3 { return true; }
    if n % 2 == 0 { return false; }
    if n % 3 == 0 { return false; }
    if n % 1 > 0 { return false; }
    let sqrtOfN: i32 = (n as f64).sqrt().round() as i32;
    let mut i: i32 = 5;
    loop {
        if n % i == 0 { return false; }
        if n % (i + 2) == 0 { return false; }
        if i > sqrtOfN {
            break;
        }
        i += 6;
    }
    return true;
}

fn main() {
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    let start = SystemTime::now();
    let time = start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    loop {
        if isPrime(i) {
            count += 1;
        }
        if i >= 10000000 {
            break;
        }
        i += 1;
    }
    println!("{:?}", (SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis() - time) as f32 / 1000.0);
}