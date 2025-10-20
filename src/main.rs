use std::fs::read_to_string;
use std::*;
use std::iter::*;
use ndarray::*;
use itertools::Itertools;

fn main() {
    // parse the input
    let mut total = 0;
    let lines = read_to_string("./input.txt").unwrap();
    // make a mask for easy compare
    let xmas = Array::from_vec(vec!['M','A','S']);
    // now dirs must be an x, so remove any item with 0
    let dirs = Array::from_vec((-1..=1).cartesian_product(-1..=1).filter(|(x,y)| *x != 0 && *y !=0 ).collect::<Vec<_>>());
    // fill the array 
    let rows = lines.lines().count();
    let mut cols = 0;
    let mut inp = Vec::<char>::new();
    for e in lines.lines() {
        // messy but works
        cols = e.len();
        for c in e.chars() {
            inp.push(c);
        }
    }
    // nono
    if cols == 0 || rows == 0 {
        panic!("cols or rows is 0")
    }

    let grid = Array::from_shape_vec((rows, cols), inp).unwrap();
    // println!("{:?}", (rows, cols));
    // println!("{:?}", grid.slice(s![0..1; -1, 0..4; -1]));

    for ((y,x), char) in grid.indexed_iter() {
        // A marks the spot
        // check for equality if the endpoint is in bounds
        if *char != 'A' || y == rows -1 || y == 0 || x == 0 || x == cols -1 {
            continue;
        }
        // start on (y,x), and add offset
        // this is the 3*3
        let slice = grid.slice(s![y-1..=y+1, x-1..=x+1]);


        let (diag, anti_diag) = (slice.diag(), Array::from_vec(vec![slice[[0,2]], slice[[1,1]], slice[[2,0]]])) ;
        if (diag == xmas || diag == xmas.slice(s![..; -1])) &&
           (anti_diag == xmas || anti_diag == xmas.slice(s![..; -1])) {
            total +=1;
        }
    }
    // println!("{:?}", grid);
    println!("total {total}");
}


