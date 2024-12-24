use std::collections::HashMap;

fn is_not_valid(probe: &[String], rules: &Vec<String>) -> bool {
    rules.iter().any(|r| probe.iter().any(|p| p == r))
}

pub fn solve(rules: &mut HashMap<String, Vec<String>>, updates: &Vec<Vec<String>>) -> Vec<Vec<String>>{
    let mut sum = 0;
    let mut is_valid = true;
    let mut updates_not_valid = Vec::new();

    for update in updates {
        for (i, page) in update.iter().enumerate() {
            if is_not_valid(
                &update[0..i],
                &rules.entry(page.to_string()).or_insert(Vec::new()),
            ) {
                updates_not_valid.push(update.clone());
                is_valid = false;
                break;
            }
        }
        if is_valid {
            let mid = update.len() / 2;
            sum += update[mid].parse::<i32>().unwrap();
        }

        is_valid = true;
    }

    println!("Sum pt1 : {}", sum);
    updates_not_valid
}
