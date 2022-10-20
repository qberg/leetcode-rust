pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        let n = candidates.len();
        let mut res: Vec<Vec<i32>> = vec![];
        let mut selection: Vec<i32> = vec![];
        Self::backtrack(0, &candidates, &mut res, &mut selection, target);
        res
    }

    fn backtrack (
        first: usize,
        candidates: &[i32],
        res: &mut Vec<Vec<i32>>,
        selection: &mut Vec<i32>,
        target: i32,
    ) {
        if target == 0 {
            res.push(selection.to_vec());
            return;
        }
        for i in first..candidates.len() {
            if candidates[i] > target {
                return;
            }
            selection.push(candidates[i]);
            Self::backtrack(i,candidates, res, selection, target - candidates[i]);
            selection.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![3, 2, 2],]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![5, 3], vec![3, 3, 2], vec![2, 2, 2, 2],]
        );
    }
}
