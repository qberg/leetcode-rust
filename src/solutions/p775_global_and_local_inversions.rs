pub struct Solution {}

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut max: i32 = -1;
        if n < 3 {
            return true;
        }
        for i in 0..n-2 {
            max = i32::max(max,nums[i]);
            if nums[i+2] > max {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_775 () {
        assert_eq!(Solution::is_ideal_permutation(vec![0]), true);
        assert_eq!(Solution::is_ideal_permutation(vec![1,2,0]), false);
    }
}
