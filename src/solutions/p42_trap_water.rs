// Based on neetcode...
pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len() as usize;
        let mut res: i32 = 0;
        if n == 0 {
            return res;
        }
        let mut low: usize = 0;
        let mut high = n-1;
        let mut level = i32::min(low as i32,high as i32);
        while low < high {
            if height[low] < height[high] {
                level = i32::max(level,height[low]);
                res += level - height[low];
                low += 1;
            } else {
                level = i32::max(level,height[high]);
                res += level - height[high];
                high -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_41 () {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        assert_eq!(Solution::trap(height),6);
    }
}
