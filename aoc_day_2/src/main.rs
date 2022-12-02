use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input should have been read");
    // println!("input: \n{input}");
    let items = input.split("\n");

    //A Rock, B Paper, C Scissors

    //x lose, y draw, z win

    let mut total_score = 0;
    for item in items {
        let score = match item {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => 0,
        };

        total_score += score;
    }

    println!("{total_score}");
}
