pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums = nums.into_iter().filter(|&num| num>0).collect();
        let n = nums.len();
        // Case: Nums contains no elements or no positive elements.
        if n == 0 {
            return 1;
        }
        for i in 0..n {
            let idx = i32::abs(nums[i]) as usize;
            if idx < n+1 && nums[idx-1] > 0 {
                nums[idx-1] = -nums[idx-1];
            }
        }
        for i in 0..n {
            if nums[i] > 0 {
                return (i+1) as i32;
            }
        }
        return (n+1) as i32; 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41 () {
        assert_eq!(Solution::first_missing_positive(vec![1,2,0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
    }
}
