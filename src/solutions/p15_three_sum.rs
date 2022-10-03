pub struct Solution{}

impl Solution{
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let n = nums.len();
        if n < 3 {
            return res;
        }
        let mut nums = nums;
        nums.sort();
        // Check for existence of negative numbers.
        if nums[0] >= 0 {
            return res;
        }
        for i in 0..n-2 {
            if i>0 && nums[i] == nums[i-1] {
                continue;
            }
            let target: i32 = -nums[i];
            let two_sum_res = Solution::two_sum(&nums[(i+1)..n],target);
            for tuple in two_sum_res.iter() {
                res.push(vec![nums[i],tuple.0,tuple.1]);
            }
        } 
        return res;
    }
    fn two_sum (nums: &[i32], target: i32) -> Vec<(i32,i32)> {
        let mut low: usize = 0;
        let mut high = nums.len() - 1;
        let mut res: Vec<(i32,i32)> = vec![];
        let mut sum: i32;
        while low < high {
            sum = nums[low] + nums[high];
            if sum == target {
                res.push((nums[low],nums[high]));
                while low < high && nums[low] == nums[low+1] { 
                    low += 1;
                }
                while low < high && nums[high] == nums[high-1] {
                    high -= 1;
                }
                low  += 1;
                high -= 1;
            } else if sum < target {
                low += 1;
            } else {
                high -= 1;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15 () {
        let nums = vec![-1,0,1,2,-1,-4];
        let answer = vec![vec![-1,-1,2], vec![-1,0,1]];
        assert_eq!(Solution::three_sum(nums),answer);
    }
}
