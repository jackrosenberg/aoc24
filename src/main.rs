use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {
    let mut n = String::new();

    println!("Enter num");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n = n.trim().parse::<u32>().expect("Parse failed");
    println!("The result is {}", fibr(n));
}

// // takes an n, rets the nth fib
// fn fib(n: u32) -> u32 {
//     let mut i = 0;
//     let mut num : (u32, u32) = ( 0, 1 );
//     loop {
//         if i == n { break; }
//         num = ( num.1, num.0 + num.1);
//         i+=1;
//     }
//     num.0
// }


fn fibr(n: u32) -> u32 {
    if n < 2 {
       return n;
    }
    fibr(n-2) + fibr(n-1)
}
