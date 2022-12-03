use std::{collections::HashSet, fs, hash::Hash};

// fn main() {
//     let input = fs::read_to_string("input.txt").expect("input should have been read");
//     // println!("input: \n{input}");
//     let items = input.split("\n");

//     let mut total = 0;

//     for item in items {
//         //let chars: HashSet<char> = item.chars().collect();
//         let length = item.len() / 2;

//         let (first, second) = item.split_at(length);
//         let h_first: HashSet<_> = first.chars().collect();
//         let h_second: HashSet<_> = second.chars().collect();

//         // let uniques = h_first.intersection(&h_second)

//         for dup in h_first.intersection(&h_second) {
//             let value = dup.clone() as u32;
//             let modifer = match dup.is_ascii_uppercase() {
//                 true => 38,
//                 false => 96,
//             };

//             println!("item: {item}, first: {first}, second: {second}, char: {dup}, mod: {modifer}, value: {value}");

//             total += value - modifer;
//         }
//     }

//     println!("{total}");

//     //as u32 - 96 if lower, -38 if upper
//     //97-122 a - z (1-26) -96
//     //65 -90 A - Z (27-52) -38
//     //char.is_ascii_uppercase
//     //char.is_ascii.lowercase
// }

fn main() {
    let input = fs::read_to_string("input.txt").expect("input should have been read");
    // println!("input: \n{input}");
    let items: Vec<&str> = input.split("\n").into_iter().collect();
    let chunks = items.chunks(3);

    let mut total = 0;

    for group in chunks {
        let first: HashSet<_> = group[0].chars().collect();
        let second: HashSet<_> = group[1].chars().collect();
        let third: HashSet<_> = group[2].chars().collect();

        let first_second: HashSet<_> = first.intersection(&second).map(|v| *v).collect();
        let last = first_second.intersection(&third);

        for dup in last {
            let value = dup.clone() as u32;
            let modifer = match dup.is_ascii_uppercase() {
                true => 38,
                false => 96,
            };

            total += value - modifer;
        }
    }

    println!("{total}")
}
