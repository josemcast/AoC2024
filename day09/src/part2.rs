use std::{collections::HashMap, usize};

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

fn find_space(disk_map: &Vec<String>, limit: usize) -> HashMap<usize, Vec<usize>> {
    let mut i = 0;

    let mut spaces = HashMap::new();
    while i < limit {
        if disk_map[i] == "." {
            let free_space = (1..10)
                .find_map(|n| {
                    if (i + n) < disk_map.len() {
                        if disk_map[i + n] != "." {
                            return Some(n);
                        }
                    }
                    None
                })
                .unwrap_or(0);

            spaces.entry(free_space).or_insert(Vec::new()).push(i);
            i += free_space;
        }
        i += 1;
    }

    spaces
}

fn defrag(disk_map: &mut Vec<String>, id_blocks: &HashMap<String, u32>) {
    let mut end = disk_map.len() - 1;

    let mut spaces = find_space(disk_map, disk_map.len());

    while disk_map[end] != "0" {
        if disk_map[end] == "." {
            end -= 1;
        } else {
            let block_size = id_blocks[&disk_map[end]] as usize;

            let mut free_size = block_size;
            let mut candidates = Vec::new();
            while free_size < 10 {
                let locations = spaces.get(&free_size).unwrap_or(&Vec::new()).clone();

                if locations.len() > 0 {
                    candidates.push((free_size, locations[0]));
                    free_size += 1;
                    continue;
                }
                free_size += 1;
            }

            if candidates.len() < 1 {
                end -= block_size as usize;
                continue;
            } else {
                if candidates.len() == 1 {
                    free_size = candidates[0].0;
                } else {
                    free_size = candidates[0].0;
                    let mut last_index = candidates[0].1;
                    for index in 1..candidates.len() {
                        if candidates[index].1 < last_index {
                            free_size = candidates[index].0;
                            last_index = candidates[index].1;
                        }
                    }
                }
                let locations = spaces.get(&free_size).unwrap_or(&Vec::new()).clone();
                for (j, index) in locations.iter().enumerate() {
                    if *index > end {
                        continue;
                    } else {
                        for i in 0..block_size {
                            let aux = disk_map[end - i as usize].to_string();
                            disk_map[end - i as usize] = disk_map[index + i as usize].to_string();
                            disk_map[index + i as usize] = aux;
                        }

                        let v = spaces.entry(free_size).or_insert(Vec::new());
                        v.remove(j);
                        if free_size > block_size {
                            let v = spaces.entry(free_size - block_size).or_insert(Vec::new());
                            if v.len() > 0 {
                                v.push(index + block_size);
                                v.sort();
                            } else {
                                v.push(index + block_size);
                            }
                        }
                        break;
                    }
                }
                end -= block_size;
            }
            //println!("spaces: {:?}", spaces);
            //println!("{} - {end}", disk_map[end]);
        }
    }
    //println!("{:?}", disk_map);
}

pub fn solve(disk_map: &mut Vec<String>, id_blocks: &HashMap<String, u32>) {
    defrag(disk_map, id_blocks);
    let checksum = check_sum(disk_map);

    println!("checksum pt2: {checksum}");
}
