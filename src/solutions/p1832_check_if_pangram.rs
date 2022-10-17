pub struct Solution {}

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut set = std::collections::HashSet::new();
        for c in sentence.chars() {
            set.insert(c);
        }
        set.len() == 26
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1832 () {
        let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
        assert_eq!(
            Solution::check_if_pangram(sentence),
            true
        );
        let sentence = "leetcode".to_string();
        assert_eq!(
            Solution::check_if_pangram(sentence),
            false
        );
    }
}
