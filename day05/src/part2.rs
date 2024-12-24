use std::collections::HashMap;

fn is_not_valid(probe: &[String], rules: &Vec<String>) -> (bool, usize) {
    for  r in rules {
        for (i, p) in probe.iter().enumerate() {
            if p == r {
                return (true, i);
            }
        }
    }
    (false, 0)
}

pub fn solve(rules: &mut HashMap<String, Vec<String>>, updates: &mut Vec<Vec<String>>) {
    let mut sum = 0;
    //let mut updates_not_valid = Vec::new();

    let mut i = 1;
    for not_valid in updates {

        let (mut pred, mut index) =  is_not_valid(
            &not_valid[0..i],
            &rules.entry(not_valid[i].to_string()).or_insert(Vec::new()),
        );

        while i != not_valid.len() {
            if pred {
                let aux = not_valid[i].to_string();
                not_valid[i] = not_valid[index].to_string();
                not_valid[index] = aux;
                i = 0;
            }
            i += 1;
            if i < not_valid.len() {
                (pred, index) = is_not_valid(
                    &not_valid[0..i],
                    &rules.entry(not_valid[i].to_string()).or_insert(Vec::new()),
                );
            }
            
        }
        let mid = not_valid.len() / 2;
        sum += not_valid[mid].parse::<i32>().unwrap();
        i = 0;
    }

    println!("Sum pt2 : {} ", sum);
}
