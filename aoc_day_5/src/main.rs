use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("input should have been read");
    let moves: Vec<&str> = input.split("\n").into_iter().collect();

    let beginning = fs::read_to_string("input.txt").expect("input should have been read");
    let state: Vec<&str> = beginning.split("\n").into_iter().collect();

    let mut stacks: HashMap<u16, Vec<&str>> = HashMap::from([
        (1, Vec::new()),
        (2, Vec::new()),
        (3, Vec::new()),
        (4, Vec::new()),
        (5, Vec::new()),
        (6, Vec::new()),
        (7, Vec::new()),
        (8, Vec::new()),
        (9, Vec::new()),
    ]);

    //create a ds here to hold the split out crate values
    //just iterating and pushing won't work as it will be dropped after the loop and need to have that memory allocated still in order to push
    //This was a bit of a brainfuck
    //I can understand why memory management is such a clusterfuck in c.

    //[[z,g,v,v,q,m,l,n,r], [j,s,q,s,z,w,p,g,r], etc]
    let mut crate_info: Vec<Vec<String>> = Vec::new();

    for row in state.into_iter().rev() {
        let crates: Vec<String> = row
            .split(" ")
            .map(|v| v.replace("[", "").replace("]", ""))
            .collect();

        crate_info.push(crates);
    }

    //now push the values from crate_info into the final stacks(our working set)
    for row in crate_info {
        for (pos, entry) in row.iter().enumerate() {}
    }
}
