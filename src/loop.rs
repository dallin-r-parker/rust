use std::{i32};


fn main() {
    let mut x = 1;

    loop {
        if x % 2 == 0 {
            println!("{}", x);
            x += 1;

            continue;
        }

        if x > 10 { break }
        x += 1;
        continue;
    }

    let mut y = 1;

    while y <= 10 {
        println!("{}", y);
        y += 1;
    }
    let start:i32 = 1;
    let end:i32 = 10;

    for z in start..end {
        println!("FOR : {}", z);
    }


}