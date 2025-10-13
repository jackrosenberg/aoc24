use std::fs::read_to_string;
use std::*;

fn main() {
    // parse the input
    let mut total = 0;
    let lines = read_to_string("./input.txt").unwrap();
    let mut do_me = true;
    for line in lines.lines()
    // there has to be a way to make this prettier
    {
        for i in 0..line.len() {
            if let Some((res, mut rest)) = parse_word(&line[i..], "mul(") {
                if let Some((f, rest)) = parse_digits(rest) {
                    if let Some((_, rest)) = parse_char(rest, ',') {
                        if let Some((s, rest)) = parse_digits(rest) {
                            if let Some((_, _rest)) = parse_char(rest, ')') {
                                println!("mul({f},{s})");
                                total += f * s;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("total {total}");
    // println!("parse 104 {:?}", parse_digits("104"));
    
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
    let mut chars = target.chars();
    let target_char = chars.next()?;
    let (char, rest) = parse_char(input, target_char)?;
    if let Some((r_chars, r_rest)) = parse_word(rest, &chars.collect::<String>()) {
        Some((char.to_string() + &r_chars, r_rest))
    }
    else { Some((char.to_string(), rest)) }
}
