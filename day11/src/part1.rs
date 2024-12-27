use std::collections::HashMap;

fn blink(stone: i64, count: &mut HashMap<(i64, i32), i64>, nblinks: i32) -> i64 {
    if nblinks < 0 {
        return 1;
    }

    let mut sum = 0;
    if nblinks >= 0 {
        sum += match count.get(&(stone, nblinks)) {
            Some(&num) => num,
            None => 0,
        };

        if sum != 0 {
            return sum;
        }

        if stone == 0 {
            sum += blink(1, count, nblinks - 1);
            *count.entry((0, nblinks)).or_insert(0) = sum;
        } else if stone.to_string().chars().count() % 2 == 0 {
            let mut first = stone.to_string();
            let second = first.split_off(first.len() / 2);
            let first = first.parse::<i64>().unwrap();
            let second = second.parse::<i64>().unwrap();
            sum += blink(first, count, nblinks - 1);
            //*count.entry((first, nblinks)).or_insert(0) = sum;
            sum += blink(second, count, nblinks - 1);
            *count.entry((stone, nblinks)).or_insert(0) = sum;
        } else {
            sum += blink(stone * 2024, count, nblinks - 1);
            *count.entry((stone, nblinks)).or_insert(0) = sum;
        }
    }
    sum
}

pub fn solve(stones: &Vec<i64>) {
    let mut count = 0; //blink(stones[0], 24);
    let mut cache_count = HashMap::new();
    for &ele in stones {
        count += blink(ele, &mut cache_count, 24);
    }

    println!("Stones count 25 blinks: {}", count);
}
