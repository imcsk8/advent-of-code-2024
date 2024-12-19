use std::fs;
use sorting_rs;
use std::collections::HashMap;

static INPUT_DATA: &str = "../data/input";

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut distance: Vec<i32> = Vec::new();
    let mut appearances: HashMap<i32, i32> = HashMap::new();
    for line in fs::read_to_string(INPUT_DATA).unwrap().lines() {
        let mut iter = line.split_ascii_whitespace();
        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        right.push(iter.next().unwrap().parse::<i32>().unwrap());
        //total += strnum.parse::<i32>().unwrap();
    }
    sorting_rs::quick_sort(&mut left);
    sorting_rs::quick_sort(&mut right);
    let mut total = 0;
    let mut total_similarity = 0; 
    for (i, _) in left.iter().enumerate() {
        let count = count_right(left[i], &right, &mut appearances);
        let similarity = left[i] * count;
        let dist = (left[i] - right[i]).abs();
        total += dist;
        total_similarity += similarity;
        distance.push(dist);
        println!("LEFT: {}, RIGHT: {}, DISTANCE: {}, COUNT: {}, SIMILARITY: {}",
            left[i], right[i], dist, count, similarity);
    }
    println!("Total Distance: {}", total);
    println!("Total Similarity: {}", total_similarity);
}

// This is going to be very slow
fn count_right(left_elem: i32, right: &Vec<i32>, ap: &mut HashMap<i32, i32>) -> i32 {
    let mut count = 0;
    for r in right {
        if ap.contains_key(&left_elem) {
            return *ap.get(&left_elem).unwrap();
        }
        if left_elem == *r {
            count += 1;
        }
    }
    ap.insert(left_elem, count);
    count
}
