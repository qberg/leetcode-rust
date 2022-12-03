pub struct Solution {}

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut nums: Vec<i32> = (1..=n).collect();
        let mut res: i32 = 0;
        Self::backtrack(0, &mut nums, &mut res, n as usize);
        res
    }

    fn backtrack (
        first: usize,
        nums:  &mut Vec<i32>,
        res:   &mut i32,
        n:     usize,
    ) {
        if first == n {
            *res += 1;
        } else {
            for i in first..n {
                let num = nums[i];
                let pos = first as i32 + 1;
                if num%pos != 0 && pos%num != 0 {
                    continue;
                }
                nums.swap(first,i);
                Self::backtrack(i+1, nums, res, n);
                nums.swap(first,i);
            }
        }
    }
}
