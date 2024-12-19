use std::fs;
use sorting_rs;

static INPUT_DATA: &str = "../data/input";

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut distance: Vec<i32> = Vec::new();
    for line in fs::read_to_string(INPUT_DATA).unwrap().lines() {
        let mut iter = line.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        right.push(iter.next().unwrap().parse::<i32>().unwrap());
        //total += strnum.parse::<i32>().unwrap();
    }
    sorting_rs::quick_sort(&mut left);
    sorting_rs::quick_sort(&mut right);
    let mut total = 0;
    for (i, _) in left.iter().enumerate() {
        let dist = (left[i] - right[i]).abs();
        total += dist;
        distance.push(dist);
        println!("LEFT: {}, RIGHT: {}, DISTANCE: {}", left[i], right[i], dist);
    }
    println!("Total Distance: {}", total);
}
