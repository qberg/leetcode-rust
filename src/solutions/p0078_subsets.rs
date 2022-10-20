pub struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res: Vec<Vec<i32>> = vec![vec![]];
        for target_size in 1..=n {
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
    fn test_78() {
        assert_eq!(
            Solution::subsets(vec![]), 
            vec![vec![]]
        );

        assert_eq!(
            Solution::subsets(vec![1]),
            vec![vec![],
            vec![1]]
        );

        assert_eq!(
            Solution::subsets(vec![1, 2]),
            vec![vec![], vec![2], vec![1], vec![1, 2]]
        );
    }
}
