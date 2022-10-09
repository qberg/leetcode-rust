pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 1 {
            return n == 1;
        }
        n % 3 == 0 && Solution::is_power_of_three(n/3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_231() {
        assert_eq!(Solution::is_power_of_three(-1), false);
        assert_eq!(Solution::is_power_of_three(1), true);
        assert_eq!(Solution::is_power_of_three(27), true);
    }
}
