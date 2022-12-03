use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let letter_counts: HashMap<char, usize> = num
            .chars()
            .fold(HashMap::new(), |mut hash_map, c| {
                  *hash_map.entry(c).or_insert(0) += 1;
                  hash_map
            });
        let res = String::new();
        let mut left_string = String::new();
        let mut right_string = String::new();
        for n in (0..=9).rev() {
            let c = char::from_digit(n, 10).unwrap();
            match letter_counts.get(&c) {
                Some(&v) => {
                    if c == '0' && left_string.is_empty() {
                        continue;
                    }
                    while v > 1 {
                        left_string.push(c);
                        right_string.push(c);
                        v -= 2;
                    }
                },
                None => {},
            };
        }
        for n in (0..=9).rev(){
            break;
        }
        return "0".to_string();
    }
}
