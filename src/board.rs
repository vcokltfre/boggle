#[derive(Debug)]
pub struct Board {
    letters: [char; 16],
    counts: [u8; 26],
    adjacents: Vec<Vec<usize>>,
}

impl Board {
    pub fn new(letters: [char; 16]) -> Board {
        let mut counts = [0; 26];

        for c in letters.iter() {
            let index = (c.to_ascii_lowercase() as u8) - 97;
            counts[index as usize] += 1;
        }

        // 0  1  2  3
        // 4  5  6  7
        // 8  9  10 11
        // 12 13 14 15

        let adjacents: Vec<Vec<usize>> = vec![
            vec![1, 4, 5],                    // 0
            vec![0, 2, 4, 5, 6],              // 1
            vec![1, 3, 5, 6, 7],              // 2
            vec![2, 6, 7],                    // 3
            vec![0, 1, 5, 8, 9],              // 4
            vec![0, 1, 2, 4, 6, 8, 9, 10],    // 5
            vec![1, 2, 3, 5, 7, 9, 10, 11],   // 6
            vec![2, 3, 6, 10, 11],            // 7
            vec![4, 5, 9, 12, 13],            // 8
            vec![4, 5, 6, 8, 10, 12, 13, 14], // 9
            vec![5, 6, 7, 9, 11, 13, 14, 15], // 10
            vec![6, 7, 10, 14, 15],           // 11
            vec![8, 9, 13],                   // 12
            vec![8, 9, 10, 12, 14],           // 13
            vec![9, 10, 11, 13, 15],          // 14
            vec![10, 11, 14],                 // 15
        ];

        Board {
            letters,
            counts,
            adjacents,
        }
    }

    pub fn new_from_string(s: &str) -> Board {
        let mut letters = [0 as char; 16];

        for (i, c) in s.chars().enumerate() {
            letters[i] = c;
        }

        Board::new(letters)
    }

    fn word_maybe_possible(&self, word: &str) -> bool {
        let mut counts = [0; 26];

        for c in word.chars() {
            let index = (c.to_ascii_lowercase() as u8) - 97;
            counts[index as usize] += 1;
        }

        for (i, item) in counts.iter().enumerate() {
            if item > &self.counts[i] {
                return false;
            }
        }

        true
    }

    fn get_adjacent_letters(&self, index: usize) -> Vec<(char, usize)> {
        self.adjacents[index]
            .iter()
            .map(|i| (self.letters[*i], *i))
            .collect()
    }

    fn recurse_possible(&self, word: &str, at: usize, visited: [bool; 16]) -> bool {
        if word.len() == 1 {
            return true;
        }

        let mut new_visited = visited;
        new_visited[at] = true;

        let mut possible = false;

        for (c, i) in self.get_adjacent_letters(at) {
            if c.to_ascii_lowercase() == word.chars().nth(1).unwrap() && !new_visited[i] {
                possible = self.recurse_possible(&word[1..], i, new_visited);
            }
        }

        possible
    }

    fn find_word(&self, word: &str, at: usize) -> bool {
        let mut visited = [false; 16];
        visited[at] = true;

        self.recurse_possible(word, at, visited)
    }

    fn word_is_possible(&self, word: &str) -> bool {
        for (i, c) in self.letters.iter().enumerate() {
            if c.to_ascii_lowercase() == word.chars().next().unwrap() && self.find_word(word, i) {
                return true;
            }
        }

        false
    }

    pub fn find_words(&self, dictionary: Vec<String>) -> Vec<String> {
        dictionary
            .into_iter()
            .filter(|word| self.word_maybe_possible(word))
            .filter(|word| self.word_is_possible(word))
            .collect()
    }
}
