/*
Given a list of integers, use a vector and return the mean (average), median
(when sorted, the value in the middle position), and mode (the value that
occurs most often; a hash map will be helpful here) of the list.
*/
use std::collections::HashMap;

#[derive(Debug)]
pub struct Properties {
    vector: Vec<i32>,
    mean: Option<f64>,
    median: Option<i32>,
    mode: Option<i32>,
}

pub fn get_properties(list: &[i32]) -> Properties {
    Properties {
        vector: list.iter().cloned().collect(),
        mean: mean(&list),
        median: median(&list),
        mode: mode(&list),
    }
}

fn mean(list: &[i32]) -> Option<f64> {
    if list.len() == 0 {
        return None;
    } else {
        let sum: i32 = list.iter().sum();
        return Some(sum as f64 / list.len() as f64);
    }
}

fn median(list: &[i32]) -> Option<i32> {
    let mut copy: Vec<i32> = list.iter().cloned().collect();
    copy.sort();
    copy.get((copy.len() / 2)).map(|&x| x)
}

fn mode(list: &[i32]) -> Option<i32> {
    let mut occurences = HashMap::new();
    for number in list {
        let count = occurences.entry(number).or_insert(0);
        *count += 1;
    }
    let max: Option<i32> = occurences.values().max().map(|&x| x);
    if max.is_some() {
        occurences.retain(|_, &mut v| v == max.unwrap());
        occurences.keys().nth(0).map(|&&x| x)  // return the first occurence
    } else {
        None
    }
}
