pub struct Solution {}

impl Solution {
    pub fn pivot_index (nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        let mut left_sum: i32 = 0;
        for num in nums.iter() {
            sum += *num;
        }
        for idx in 0..n {
            if left_sum == sum - left_sum - nums[idx] {
                return idx as i32;
            }
            left_sum += nums[idx];
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_724 () {
        assert_eq!(Solution::pivot_index(vec![1,7,3,6,5,6]), 3);
        assert_eq!(Solution::pivot_index(vec![1,2,3]), -1);
    }
}
