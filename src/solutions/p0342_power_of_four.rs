pub struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 1 {
            return n == 1;
        }
        n % 4 == 0 && Solution::is_power_of_four(n/4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_241() {
        assert_eq!(Solution::is_power_of_four(-1), false);
        assert_eq!(Solution::is_power_of_four(1), true);
        assert_eq!(Solution::is_power_of_four(16), true);
    }
}
