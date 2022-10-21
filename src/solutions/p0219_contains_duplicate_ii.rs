pub struct Solution {}

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        for left in 0..n {
            let right = usize::min(n-1, left+(k as usize));
            let mut subset: Vec<i32> = nums[left..=right].to_vec();
            if Self::contains_duplicate(&mut subset) {
                return true;
            }
        }
        false
    }

    fn contains_duplicate (subset: &mut[i32]) -> bool {
        subset.sort_unstable();
        for i in 1..subset.len() {
            if subset[i] == subset[i-1] {
                return true;
            }
        }
        false
    }
}
