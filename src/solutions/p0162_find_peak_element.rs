pub struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n==1 {
            return 0;
        }
        let (mut low, mut high) = (0_usize, n - 1);
        let mut mid = 0;
        while low < high {
            mid = (high - low) / 2 + low;
            if mid + 1 < n && nums[mid] < nums[mid + 1] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        let arr = vec![1,2,3,1];
        assert_eq!(
            Solution::find_peak_one_dim(&arr),
            2
        );

        let arr = vec![1,2,1,3,5,6,4]; 
        assert_eq!(
            Solution::find_peak_one_dim(&arr),
            5
        );
    }
}
