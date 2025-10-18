use std::fs::read_to_string; use std::*;

fn main() {
    // parse the input
    let mut total = 0;
    let lines = read_to_string("./input.txt").unwrap();
    let mut do_me = true;
    // there has to be a way to make this prettier
    for line in lines.lines() {
        for i in 0..line.len() {
            if parse_word(&line[i..], "do()").is_some() {
                do_me = true;
            }
            if parse_word(&line[i..], "don't()").is_some() {
                do_me = false;
            }
            if let Some((res, mut rest)) = parse_word(&line[i..], "mul(") {
                if let Some((f, rest)) = parse_digits(rest) {
                    if let Some((_, rest)) = parse_char(rest, ',') {
                        if let Some((s, rest)) = parse_digits(rest) {
                            if let Some((_, _rest)) = parse_char(rest, ')') {
                                println!("mul({f},{s}) {}", do_me);
                                if do_me {
                                    total += f * s;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total {total}");
}

// general idea, parser combinators
fn parse_digit(input: &str) -> Option<(usize, &str)> {
    let mut chars = input.chars();
    let char = chars.next()?;
    match char {
        '0' => Some((0, &input[1..])),
        '1' => Some((1, &input[1..])),
        '2' => Some((2, &input[1..])),
        '3' => Some((3, &input[1..])),
        '4' => Some((4, &input[1..])),
        '5' => Some((5, &input[1..])),
        '6' => Some((6, &input[1..])),
        '7' => Some((7, &input[1..])),
        '8' => Some((8, &input[1..])),
        '9' => Some((9, &input[1..])),
        _    => None
    }
}

fn parse_digits(input: &str) -> Option<(usize, &str)> { 
    // yep, its small brain time :(
    let mut res = 0;
    let mut final_rest = input;
    while let Some((int, rest)) = parse_digit(final_rest) {
        res = 10* res + int;
        final_rest = rest;
    }
    Some((res, final_rest))
}

fn parse_char(input: &str, target: char) -> Option<(char, &str)> { 
    let char = input.chars().next()?;
    if char == target {
        return Some((char, &input[1..]));
    }
    None
}

// what the fuck is a lifetime
// for the love of god please give me monads
fn parse_word<'a>(input: &'a str, target: &str) -> Option<(String, &'a str)> {
    if target.len() > input.len() {
        return None;
    }
    if input[..target.len()] == *target {
        return Some((target.to_string(), &input[target.len()..]));
    } 
    None
}
