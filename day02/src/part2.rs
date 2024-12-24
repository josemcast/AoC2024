fn is_safe(v: &[u32]) -> bool {
    if v[0] == v[1] {
        return false;
    } else if v[0] > v[1] {
        if v[0].abs_diff(v[1]) > 3 || v[0].abs_diff(v[1]) < 1 {
            return false;
        }

        for i in 1..(v.len() - 1) {
            if v[i] < v[i + 1] {
                return false;
            } else if v[i].abs_diff(v[i + 1]) < 1 || v[i].abs_diff(v[i + 1]) > 3 {
                return false;
            }
        }
    } else {
        if v[0].abs_diff(v[1]) > 3 || v[0].abs_diff(v[1]) < 1 {
            return false;
        }

        for i in 1..(v.len() - 1) {
            if v[i] > v[i + 1] {
                return false;
            } else if v[i].abs_diff(v[i + 1]) < 1 || v[i].abs_diff(v[i + 1]) > 3 {
                return false;
            }
        }
    }

    true
}

pub fn solve(reports: &Vec<String>) {
    let mut count = 0;

    for report in reports {
        let numbers: Vec<u32> = report
            .split(" ")
            .map(|n| n.parse().expect("Could not parse"))
            .collect();

        if is_safe(&numbers) {
            count += 1;
        } else if is_safe(&numbers[1..]) {
            count += 1;
        } else if is_safe(&numbers[..(numbers.len() - 1)]) {
            count += 1;
        } else {
            for item in 1..(numbers.len() - 1) {
                let first = &numbers[..item];
                let second = &numbers[(item + 1)..];
                let aux = [first, second].concat();
                if is_safe(&aux) {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("Count pt2: {}", count);
}