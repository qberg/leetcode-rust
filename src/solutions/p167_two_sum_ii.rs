pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_ptr: usize = 0;
        let mut right_ptr: usize = numbers.len() - 1;

        while left_ptr < right_ptr {
            let sum = numbers[left_ptr] + numbers[right_ptr];
            if sum == target {
                return vec![(left_ptr+1) as i32,(right_ptr+1) as i32];
            } 
            if sum > target {
                right_ptr -= 1;
                continue;
            }
            if sum < target {
                left_ptr += 1;
                continue;
            }
        }
        return vec![0,0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_167 () {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![1,2]);
        assert_eq!(Solution::two_sum(vec![2,3,4], 6) , vec![1,3]);
        assert_eq!(Solution::two_sum(vec![-1,0], -1), vec![1,2]);
    }
}
