use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input should have been read");
    // println!("input: \n{input}");
    let items: Vec<&str> = input.split("\n").into_iter().collect();

    let mut count = 0;

    for item in items {
        let ranges: Vec<_> = item.split(",").collect();

        let ab: Vec<_> = ranges[0].split("-").collect();
        let cd: Vec<_> = ranges[1].split("-").collect();

        let mut overlaps = false;

        let (a, b, c, d) = (
            ab[0].parse::<u32>().unwrap(),
            ab[1].parse::<u32>().unwrap(),
            cd[0].parse::<u32>().unwrap(),
            cd[1].parse::<u32>().unwrap(),
        );

        if c >= a && c <= b || d >= a && d <= b {
            overlaps = true
        } else if a >= c && a <= d || b >= c && b <= d {
            overlaps = true
        }

        if overlaps {
            count += 1;
            //println!("item: {item}, {a},{b},{c},{d}");
        } else {
            println!("item, {item}");
        }
    }

    println!("{count}")
}
