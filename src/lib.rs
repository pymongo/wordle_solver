#![cfg(test)]

const WORD_LEN: usize = 5;

struct Flag;

impl Flag {
    /// the letter is in the word and in the correct spot
    const GREEN: u8 = 0;
    /// the letter is in the word but in the wrong spot
    const YELLOW: u8 = 1;
    /// the letter is not in the word
    const GRAY: u8 = 2;
}

#[derive(Default, Clone, Copy)]
struct Entry {
    letter: u8,
    flag: u8,
    position: usize
}

fn solve(rows: Vec<[&str; WORD_LEN]>) -> String {
    const POSITION_UNKNOWN: u8 = b'U';
    use std::collections::HashSet;

    // solver context
    let mut not_contains = HashSet::new();
    // same letter may occur multi times
    let mut contains = [0_u8; 26];
    let mut correct_position = [POSITION_UNKNOWN; WORD_LEN];

    // FIXME yellow may occur multitimes, word count should reset by row
    for entry in rows.into_iter().map(|row| deserialize_row(row)).flatten() {
        match entry.flag {
            Flag::GREEN => {
                contains[(entry.letter - b'a') as usize] += 1;
                correct_position[entry.position] = entry.letter;
            },
            Flag::YELLOW => {
                contains[(entry.letter - b'a') as usize] += 1
            },
            Flag::GRAY => {
                not_contains.insert(entry.letter);
            },
            _ => unreachable!()
        }
    }

    // let mut candidates = Vec::new();
    for line_str in std::fs::read_to_string("/usr/share/dict/words").unwrap().lines() {
        if line_str.len() != WORD_LEN {
            continue;
        }
        let word = line_str.as_bytes().iter().copied().collect::<Vec<_>>();
        if !word.iter().all(|letter| letter.is_ascii_lowercase()) {
            continue;
        }

        let word_hashset = line_str.as_bytes().iter().copied().collect::<HashSet<_>>();
        if not_contains.intersection(&word_hashset).count() > 0 {
            continue;
        }
        
        let mut word_letter_count = [0_u8; 26];
        for letter in &word {
            word_letter_count[(letter - b'a') as usize] += 1;
        }
        if (0..26).any(|i| {
            contains[i] > 0 && contains[i] != word_letter_count[i]
        }) {
            continue;
        }

        if (0..WORD_LEN).any(|i| {
            correct_position[i] != POSITION_UNKNOWN && word[i] != correct_position[i]
        }) {
            continue;
        }
        return unsafe { String::from_utf8_unchecked(word) };
        // candidates.push(word);
    }
    panic!("no candidates found");
}

// can't use iterator collect to [Entry; 5], can only collect to Vec<Entry>
fn deserialize_row(row: [&str; WORD_LEN]) -> [Entry; WORD_LEN] {
    let mut row_entry = [Entry::default(); WORD_LEN];
    for i in 0..WORD_LEN {
        let item = row[i].as_bytes();
        row_entry[i] = Entry {
            letter: item[0],
            flag: item[1] - b'0',
            position: i
        };
    }
    row_entry
}

#[test]
fn solve_22_02_16() {
    dbg!(solve(vec![
        ["t2", "e2", "a1", "c1", "h2"],
    ])); 
}
