use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let len: usize = s.len();
        let vowels: HashSet<char> = HashSet::from_iter(vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']); 
        let left = &s[..len/2]
            .chars()
            .filter(|c| vowels.contains(c))
            .count();
        let right = &s[len/2..]
            .chars()
            .filter(|c| vowels.contains(c))
            .count();
        left == right 
    }
}
