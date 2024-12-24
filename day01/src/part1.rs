pub fn solve(list1: &Vec<u32>, list2: &Vec<u32>) {

    let dist = (0..list1.len())
    .fold(0, |mut acc, n| {
        acc += list1[n].abs_diff(list2[n]);
        acc
    });

    println!("Dist pt1: {}", dist);
}