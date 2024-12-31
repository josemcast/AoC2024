fn operate_machine(button_a: (f64, f64), button_b: (f64, f64), prize: (f64, f64)) -> (i64, i64) {
    let prize = (prize.0 + 10000000000000.0, prize.1 + 10000000000000.0);
    let det = button_a.0 * button_b.1 - button_b.0 * button_a.1;
    if det == 0.0 {
        return (0, 0);
    }

    let c1 = (button_b.1 * prize.0 - button_b.0 * prize.1) / det;
    let c2 = (button_a.0 * prize.1 - button_a.1 * prize.0) / det;

    if c1 >= 0.0 && c1.fract() == 0.0 && c2 >= 0.0 && c2.fract() == 0.0 {
        return (c1 as i64, c2 as i64);
    }
    (0, 0)
}
pub fn solve(machines: &Vec<(f64, f64)>) {
    let mut sum = 0;

    for machine in machines.chunks(3) {
        let (c1, c2) = operate_machine(machine[0], machine[1], machine[2]);
        sum += 3 * c1 + 1 * c2;
    }

    println!("Sum pt2. {sum}");
}
