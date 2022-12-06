use std::{collections::HashMap, fs};

fn main() {
    let beginning = fs::read_to_string("state.txt").expect("input should have been read");
    let state: Vec<&str> = beginning.split("\n").into_iter().collect();

    let mut stacks: HashMap<u32, Vec<char>> = HashMap::from([
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

    for row in state.into_iter().rev() {
        let mut mapped_row = String::from(row);
        mapped_row.push(' ');

        let mut iter_chars = mapped_row.chars();
        let char_length = mapped_row.chars().count();

        for i in (0..char_length).step_by(4) {
            iter_chars.next();
            let entry = iter_chars.next().clone().unwrap();
            iter_chars.next();
            iter_chars.next();

            if entry != ' ' {
                let stack_index = ((i + 1) as f32 / 4.0).ceil() as u32;
                //println!("stack: {stack_index}, entry: {entry}");
                let stack = stacks.get_mut(&stack_index).unwrap();
                stack.push(entry);
            }
        }
    }

    let input = fs::read_to_string("input.txt").expect("input should have been read");
    let moves: Vec<&str> = input.split("\n").into_iter().collect();

    for line in moves.into_iter() {
        let processed = line
            .replace("move ", "")
            .replace(" from ", ",")
            .replace(" to ", ",");

        let mut operations = processed.split(",");
        let to_move: u32 = operations.next().unwrap().parse().unwrap();
        let i_source: u32 = operations.next().unwrap().parse().unwrap();
        let i_dest: u32 = operations.next().unwrap().parse().unwrap();

        //pt 1
        // for _i in 0..to_move {
        //     let moved: char;
        //     {
        //         //if feels weird to have to regrab the stack I want to pop from each time here,
        //         //but maybe it's fine? In node this would be inefficent
        //         let source = stacks.get_mut(&i_source).unwrap();
        //         moved = source.pop().unwrap();
        //     }

        //     let dest = stacks.get_mut(&i_dest).unwrap();
        //     dest.push(moved);
        // }

        //pt 2

        let moved: Vec<char>;
        {
            let source = stacks.get_mut(&i_source).unwrap();
            let i_to_move = source.len() - to_move as usize;

            moved = source.split_off(i_to_move);
        }

        let dest = stacks.get_mut(&i_dest).unwrap();
        for j in moved.iter() {
            dest.push(*j);
        }
    }

    for (col_num, stack) in stacks {
        let last_val = stack.clone().pop().unwrap();
        println!("{col_num}, {last_val}")
    }
}
