use std::{collections::BTreeMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input should have been read");
    // println!("input: \n{input}");
    let items = input.split("\n");

    let mut totals: BTreeMap<i32, i32> = BTreeMap::new();
    let mut current_elf: i32 = 1;

    totals.insert(current_elf, 0);

    for item in items {
        if item == "" {
            current_elf += 1;
            //println!("new elf {current_elf}");
        } else {
            let stored = match totals.get(&current_elf) {
                None => 0,
                Some(val) => *val,
            };

            let item_value = item.parse::<i32>().unwrap();
            let new_total = stored + item_value;

            //println!("adding {item_value} to {current_elf}, old {stored}, new {new_total}");

            totals.insert(current_elf, new_total);
        }
    }

    // let mut current_max = 0;
    // let mut current_max_elf_owner = 0;

    // for (elf, total) in totals {
    //     println!("elf {elf} has {total} cals");
    //     if total > current_max {
    //         current_max = total;
    //         current_max_elf_owner = elf;
    //     }
    // }
    // println!("current max {current_max} max_elf: {current_max_elf_owner}");

    let cal_sorted: BTreeMap<&i32, &i32> = totals.iter().map(|(k, v)| (v, k)).collect();
    let mut top_total = 0;

    let top_elves = cal_sorted.iter().rev().take(3);
    //[high, elf],[next highest, elf], [next, elf]

    for (cals, elf) in top_elves {
        top_total += cals.clone();
        println!("elf {elf}, cals {cals}");
    }

    println!("{top_total}");
}
