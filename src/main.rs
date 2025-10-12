use std::io;
use std::fs::read_to_string;

fn main() {
    // parse the input
    let (mut l, mut r) = read_lines("./input.txt");
    l.sort();
    r.sort();

    let mut sum: u64 = 0;
    for (i, elem) in l.iter().enumerate() {
        sum += (elem - r.get(i).unwrap()).unsigned_abs() as u64;
    }
    println!("{}", sum);
}

fn read_lines(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        // split and then to int
        let words = line.split(" ").collect::<Vec<_>>();
        left.push(words[0].parse::<i32>().unwrap());
        right.push(words.last().unwrap().parse::<i32>().unwrap());
    }
    (left, right)
}

