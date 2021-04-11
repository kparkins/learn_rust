
use std::collections::HashMap;

pub fn mean(numbers: &Vec<i32>) -> f32 {
    let sum = numbers.iter().fold(0, |a, b| a + b) as f32;
    let length = numbers.len() as f32;
    return sum / length;
} 

pub fn median(numbers: &Vec<i32>) -> f32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    let midpoint = numbers.len() / 2 - 1;
    let mut median = numbers[midpoint] as f32;
    if numbers.len() % 2 == 0 {
        median += numbers[midpoint + 1] as f32;
        median /= 2.0;
    }
    median
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, u32> = HashMap::new();
    for &n in numbers {
        *counts.entry(n).or_insert(0) += 1
    }
    counts
        .into_iter()
        .max_by(|(_, a), (_, b)| a.cmp(&b))
        .map(|(a, _)| a)
        .expect("cannot find mode of zero length vector")
}