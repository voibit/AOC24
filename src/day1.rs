use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    let data = include_str!("../data/day1.txt");

    let mut left : Vec<i64> = Vec::new();
    let mut right : Vec<i64> = Vec::new();

    data.split('\n').map(|line| line.split_ascii_whitespace()).for_each(|line| {
        let mut it = line.into_iter();
        if let (Some(l),Some(r)) = (it.next(), it.next()) {
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
    });

    left.sort();
    right.sort();

    let sum : i64 = left.iter().zip(right.iter()).map(|(l,r)| {
        (l-r).abs()
    }).sum();

    println!("task 1: {}", sum);

    let mut map : HashMap<i64, usize> = HashMap::from_iter(left.iter().map(|value| (*value, 0)));

    for (key, value) in map.iter_mut()  {
        *value = right.iter().filter(|&right_val | right_val==key).count();
    }

    let task2 : usize = left.iter().map(|num| *num as usize * map[num]).sum();
    println!("task 2 {}", task2);

}
