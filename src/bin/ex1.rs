// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let numbers: Vec<u32> = std::iter::repeat_with(|| rand::thread_rng().gen_range(0, 10))
        .take(20)
        .collect();

    println!("numbers: {:?}, len: {}", numbers, numbers.len());

    if let Some(mean) = mean(&numbers) {
        println!("The mean is {}", mean);
    }

    if let Some(median) = median(&numbers) {
        println!("The median is {}", median);
    }

    if let Some((mode, times)) = mode(&numbers) {
        println!("The mode is {} appearing {} times", mode, times);
    }
}

fn mean(v: &Vec<u32>) -> Option<f64> {
    let len = v.len();

    if len == 0 {
        None
    } else {
        let sum: u32 = v.iter().sum();
        Some(sum as f64 / len as f64)
    }
}

fn median(v: &Vec<u32>) -> Option<f64> {
    let len = v.len();

    if len == 0 {
        None
    } else {
        let mut newv = v.clone();
        newv.sort();
        println!("sorted array: {:?}", newv);
        let middle_i: usize = newv.len() / 2;

        if len % 2 != 0 {
            Some(newv[middle_i] as f64)
        } else {
            Some((newv[middle_i - 1] as f64 + newv[middle_i] as f64) / 2.0)
        }
    }
}

fn mode(v: &Vec<u32>) -> Option<(u32, u32)> {
    if v.len() == 0 {
        return None;
    }

    let mut map = HashMap::new();
    let mut mode: (u32, u32) = (0, 0);

    for n in v.iter().cloned() {
        let count = map.entry(n).or_insert(0);
        *count += 1;

        if *count > mode.1 {
            mode = (n, *count);
        }
    }

    Some(mode)
}
