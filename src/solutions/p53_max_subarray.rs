pub struct Solution {}

impl Solution {
    pub fn max_sub_array (nums: Vec<i32>) -> i32 {
        let mut sum_cache: i32 = 0;
        let mut res: i32 = i32::MIN;
        for num in nums.iter() {
            sum_cache += *num;
            res = i32::max(sum_cache,res);
            if sum_cache < 0 {
                sum_cache = 0;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53 () {
        let nums = vec![5,4,-1,7,8];
        let res: i32 = 23;
        assert_eq!(Solution::max_sub_array(nums),res);
    }
}
