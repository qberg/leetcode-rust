pub struct Solution {}


impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = vec![];
        let (mut p1, mut p2) = (0,0);
        while p1 < nums1.len() && p2 < nums2.len() {
            let n1 = nums1[p1];
            let n2 = nums2[p2];
            match n1.cmp(&n2) {
                std::cmp::Ordering::Less    => {
                    nums.push(n1);
                    p1 += 1;
                },
                std::cmp::Ordering::Greater => {
                    nums.push(n2);
                    p2 += 1;
                },
                std::cmp::Ordering::Equal  => {
                    nums.push(n1);
                    nums.push(n2);
                    p1 += 1;
                    p2 += 1;
                },
            };
        }
        if p1 < nums1.len() {
            for &n in &nums1[p1..] {
                nums.push(n);
            }
        }
        if p2 < nums2.len() {
            for &n in &nums2[p2..] {
                nums.push(n);
            }
        }
        let l = nums.len();
        if l % 2 == 0 {
            let l  = nums.len();
            let ans = (nums[l/2] + nums[l/2 -1]) as f64;
            return ans/2_f64;
        } else {
            return nums[l/2] as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: implementation
    #[test]
    #[ignore]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
