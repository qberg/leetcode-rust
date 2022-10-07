pub struct Solution {}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        
    }

    pub fn is_palindrome(string: &str) -> bool {
        !string.chars().zip(string.chars().rev()).any(|(l,r)| l!=r)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_336 () {
        assert_eq!(Solution::palindrome_pairs(vec_string!["abcd","dcba","lls","s","sssll"]),vec![vec![0,1], vec![1,0], vec![3,2], vec![2,4]]);
    }

}
