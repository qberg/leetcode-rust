pub struct Solution {}

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if nums.len() < k || sum % k != 0 {
            return false;
        }
        let target: i32 = sum/k;

    }
}
