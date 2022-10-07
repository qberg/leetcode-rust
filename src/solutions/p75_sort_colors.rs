pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        let mut idx: usize = 0;
        if nums.len() <= 1 {
            return;
        }
        while left < right && idx <= right {
            if nums[idx] == 0 {
                nums[idx] = nums[left];
                nums[left] = 0;
                left += 1;
                idx += 1;
            } else if nums[idx] == 2 {
                nums[idx] = nums[right];
                nums[right] = 2;
                right -= 1;
            } else {
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_75 () {
        let mut input = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut input);
        let output = vec![0,0,1,1,2,2];
        assert_eq!(input, output);

        let mut input = vec![2,2];
        Solution::sort_colors(&mut input);
        let output = vec![2,2];
        assert_eq!(input, output);

        let mut input = vec![1,1];
        Solution::sort_colors(&mut input);
        let output = vec![1,1];
        assert_eq!(input, output);

        let mut input = vec![0,0];
        Solution::sort_colors(&mut input);
        let output = vec![0,0];
        assert_eq!(input, output);

        let mut input = vec![0];
        Solution::sort_colors(&mut input);
        let output = vec![0];
        assert_eq!(input, output);

        let mut input = vec![];
        Solution::sort_colors(&mut input);
        let output = vec![];
        assert_eq!(input, output);
    }
}
