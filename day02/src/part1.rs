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
        }
    }

    println!("Count pt1: {}", count);
}