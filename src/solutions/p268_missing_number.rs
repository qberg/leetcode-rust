pub struct Solution{}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut xor: i32 = 0;
        for i in 0..n {
            xor ^= (i + 1) as i32;
            xor ^= nums[i];
        }
        return xor;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_268() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }
}
