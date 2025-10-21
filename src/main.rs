use std::fs::read_to_string;
use std::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    // parse the input
    let mut total = 0;
    let lines = read_to_string("./input.txt").unwrap();
    let mut rules_done = false;
    let mut rules = HashMap::<usize, Vec<usize>>::new();
    let mut updates = Vec::<Vec<usize>>::new();
    let mut starters = Vec::<usize>::new();
    // real big brain shit, we're gonna do a topo sort to determine the 'correct' order once and
    // for all
    for line in lines.lines() {
        if line.is_empty() {
            rules_done = true;
            continue;
        }             
        if !rules_done {
            let mut num_iter = line.split("|");
            // each page knows what pages must come b4 itself
            let (b4, aftr) = (num_iter.next().unwrap().parse::<usize>().unwrap(), num_iter.next().unwrap().parse::<usize>().unwrap());
            // either insert or update
            let rule = rules.entry(aftr).or_default();
            rule.push(b4);
        }
        else {
            updates.push(line.split(",").map(|s| s.parse::<usize>().unwrap()).collect());
        }
    }

    println!("rules {rules:?}");
    println!("updates {updates:?}");
    let mut all_pages: Vec<usize> = rules.iter().flat_map(|(k, v)| { 
        let mut nv = v.clone();
        nv.push(*k);
        nv
    })
        .unique()
        .collect::<Vec<usize>>();
        all_pages.append(&mut updates.clone().into_iter().flatten().unique().collect::<Vec<_>>());
    

    for v in &all_pages {
        if !rules.contains_key(v) && !starters.contains(v) {
            starters.push(*v);
        }
    }
    // println!("starters {starters:?}");
    // println!("all {all_pages:?}");
    // println!("total {total}");
    let sorted = khan(&all_pages, rules);
    println!("sort {sorted:?}");
}

fn khan(all: &[usize], rules: HashMap::<usize, Vec<usize>>) -> Vec<usize> {
    let mut sorted = Vec::<usize>::new();
    let mut start_nodes = Vec::<usize>::new();
    // pop elems while we still have them
    while let Some(n) = start_nodes.pop() {
        sorted.push(n);
        for m in all.filter(|p| ) {
        }
    }
    todo!();
}

