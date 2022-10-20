pub struct Solution {}

impl Solution {
    pub fn combination_sum3(n: i32, k: i32) -> Vec<Vec<i32>> {
        let nums: Vec<i32> = (1..10).collect();
        let mut res: Vec<Vec<i32>> = vec![];
        let mut selection: Vec<i32> = vec![];
        Self::backtrack(0,&nums, &mut res, &mut selection, k, n as usize);
        res
    }

    fn backtrack (
        first: usize,
        nums: &[i32],
        res: &mut Vec<Vec<i32>>,
        selection: &mut Vec<i32>,
        target: i32,
        len: usize,
    ) {
        if selection.len() == len {
            if target == 0 {
                res.push(selection.to_vec());
                return;
            }
        }
        for i in first..nums.len() {
            let num = nums[i];
            if num > target {
                break;
            }
            selection.push(num);
            Self::backtrack(i+1,nums, res, selection, target - num, len);
            selection.pop();
        }
    }
}



