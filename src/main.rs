mod board;

fn main() {
    let words: Vec<String> = include_str!("./words.txt")
        .split('\n')
        .map(|s| s.trim())
        .filter(|word| word.len() >= 4)
        .filter(|word| word.chars().all(|c| c.is_ascii_lowercase()))
        .map(|s| s.to_string())
        .collect();

    let word = std::env::args().nth(1);

    if word.is_none() {
        panic!("board must be provided");
    }

    let word = word.unwrap();

    if word.len() != 16 {
        panic!("board must be 16 characters");
    }

    let board = board::Board::new_from_string(word.as_str());

    let mut found = board.find_words(words);
    found.sort_by_key(|w1| w1.len());
    found.reverse();

    for word in found {
        let points = match word.len() {
            3 => 1,
            4 => 1,
            5 => 2,
            6 => 3,
            7 => 5,
            _ => 11,
        };
        println!("{} {}", points, word);
    }
}
