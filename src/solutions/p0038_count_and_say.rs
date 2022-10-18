use itertools::Itertools;

pub struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }
        let to_say = Solution::count_and_say(n-1);
        Solution::say(to_say)
    }

    fn say (to_say: String) -> String {
        let word = to_say
            .chars()
            .group_by(|&c| c)
            .into_iter()
            .map(|(c,n)| {
                format!("{}{}", char::from_digit(n.count() as u32, 10).unwrap(),c)
            })
            .collect();
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_38() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
