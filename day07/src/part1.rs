fn is_calib(test: i64, prefix: i64, v: &[i64]) -> bool {
    if v.len() < 2 {
        if (prefix + v[0]) == test {
            return true;
        } else if (prefix * v[0]) == test {
            return true;
        }
        return false;
    }
    is_calib(test, prefix + v[0], &v[1..]) || is_calib(test, prefix * v[0], &v[1..])
}

pub fn solve(calibrations: &Vec<(i64, Vec<i64>)>) {
    let sum_calibrations: i64 = calibrations
        .iter()
        .filter(|(test, equation)| is_calib(*test, equation[0], &equation[1..]))
        .map(|(test, _)| *test)
        .sum();

    println!("Sum pt2: {}", sum_calibrations);
}
