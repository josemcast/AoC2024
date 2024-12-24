use std::collections::HashMap;

pub fn solve(list1: &Vec<u32>, list2: &Vec<u32>) {

    
    let count = (0..list1.len())
    .fold(HashMap::new(), |mut acc, n| {
        acc.entry(list2[n]).and_modify(|x| *x += 1).or_insert(1);
        acc
    });

    let dist = (0..list1.len()).fold(0, |mut acc, n| {
        acc += match count.get(&list1[n]) {
            Some(num) => list1[n] * num,
            None => 0,
        };
        acc
    });

    println!("Dist pt2: {}", dist);
}