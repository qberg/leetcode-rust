pub struct Solution {}

impl Solution {
    pub fn subsets_with_dup (nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        nums.sort();
        let mut res: Vec<Vec<i32>> = vec![];
        for target_size in 0..=n {
            let mut subset = vec![];
            Self::backtrack(&nums, &mut res, 0, &mut subset, target_size);
        }
        res 
    }

    fn backtrack (
        nums: &[i32],
        res: &mut Vec<Vec<i32>>,
        first: usize,
        subset: &mut Vec<i32>,
        target_size: usize,
    ) {
        if subset.len() == target_size {
            res.push(subset.to_vec());
            return;
        }
        for i in first..nums.len() {
            if i > first && nums[i] == nums[i-1] {
                continue;
            }
            subset.push(nums[i]);
            Self::backtrack(nums, res, i+1, subset, target_size);
            subset.pop(); 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_90() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![2],
                vec![2, 2],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
            ]
        );
        assert_eq!(Solution::subsets_with_dup(vec![1]), vec![vec![], vec![1],]);
        assert_eq!(Solution::subsets_with_dup(vec![]), vec![vec![],]);
    }
}
