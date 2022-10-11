pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum%2 != 0 {
            return false;
        }
        let target: i32 = sum/2;
        let mut dp: Vec<bool> = vec![false;target as usize +1];
        dp[0] = true;
        for num in nums {
            let num = num as usize;
            for t in (num..target as usize + 1).rev() {
                if dp[t - num as usize] {
                    dp[t] = true;
                }
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_416 () {
        assert_eq!(
            Solution::can_partition(vec![1,5,11,5]),
            true
        );
        assert_eq!(
            Solution::can_partition(vec![1,2,3,5]),
            false
        );
    }
}
