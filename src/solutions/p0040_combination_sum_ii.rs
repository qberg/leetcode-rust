pub struct Solution {}

impl Solution {
    pub fn combination_sum2 (candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let n = candidates.len();
        let mut candidates = candidates;
        candidates.sort_unstable();
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
            if i>first && candidates[i] == candidates[i-1] {
                continue;
            }
            selection.push(candidates[i]);
            Self::backtrack(i+1,candidates, res, selection, target - candidates[i]);
            selection.pop();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_40() {
        assert_eq!(
            Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![7, 1], vec![6, 2], vec![6, 1, 1], vec![5, 2, 1],]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![5], vec![2, 2, 1],]
        );
    }
}
