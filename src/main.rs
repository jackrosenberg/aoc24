use std::io;
use std::fs::read_to_string;

fn main() {
    // parse the input
    let (l, r) = read_lines("./input.txt");

    let mut sum: usize = 0;
    for (i, elem) in l.iter().enumerate() {
        sum += *elem as usize * r.iter().filter(|x| *x == elem).count();
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

