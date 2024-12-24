fn check_sum(disk_map: &Vec<String>) -> i64 {
    let mut checksum: i64 = 0;

    for i in 0..disk_map.len() {
        if disk_map[i] != "." {
            checksum += i as i64 * disk_map[i].parse::<i64>().unwrap();
        } else {
            checksum += 0;
        }
    }

    checksum
}

fn defrag(disk_map: &mut Vec<String>) {
    let mut start: usize = 0;
    let mut end = disk_map.len() - 1;

    while start < end {
        if disk_map[start] != "." {
            start += 1;
        } else {
            if disk_map[end] == "." {
                while disk_map[end] == "." {
                    end -= 1;
                }
            }
            let aux = disk_map[end].to_string();
            disk_map[end] = disk_map[start].to_string();
            disk_map[start] = aux;
            start += 1;
            end -= 1;
        }
    }
}

pub fn solve(disk_map: &mut Vec<String>) {
    defrag(disk_map);
    let checksum = check_sum(disk_map);

    println!("checksum pt1: {checksum}");
}
