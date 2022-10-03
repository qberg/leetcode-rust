pub struct Solution {}

impl Solution {
    pub fn max_area (height: Vec<i32>) -> i32 {
        let mut low: usize = 0;
        let mut high = height.len() - 1;
        let mut volume: i32 = 0;
        while low < high {
            let h = i32::min(height[low],height[high]);
            let l = (high - low) as i32;
            volume = i32::max(volume,h*l);
            if height[low] < height[high] {
                low += 1;
            } else {
                high -= 1;
            }
        }
        volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11 () {
        let height = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(Solution::max_area(height),49);
    }
}
