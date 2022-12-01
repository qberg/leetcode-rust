pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
       let n: usize = nums.len();
       let mut nums =nums;
       nums.sort_unstable();
       let mut smallest_gap = i32::MAX;
       let mut ans: i32 = 0;
       for i in 0..n-2 {
           let mut l = i + 1;
           let mut r = n - 1;
           while l < r {
               let sum = nums[i] + nums[l] + nums[r];
               if sum == target {
                   return target;
               }
               if i32::abs(sum - target) < smallest_gap {
                   smallest_gap = i32::abs(sum-target);
                   ans = sum;
               }
               if sum > target {
                   r -= 1;
               } else {
                   l += 1;
               }
           }
       }
       ans
    }
}
