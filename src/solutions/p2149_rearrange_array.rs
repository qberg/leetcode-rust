pub struct Solution {}

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n_plus: Vec<i32> = nums.iter().filter(|&&n| n>0).cloned().collect();
        let n_minus: Vec<i32> = nums.iter().filter(|&&n| n<0).cloned().collect();
        for i in 0..n_plus.len() {
            nums[2*i] = n_plus[i];
            nums[(2*i)+1] = n_minus[i];
        }
    return nums;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2149 () {
        assert_eq!(Solution::rearrange_array(vec![3,1,-2,-5,2,-4]), vec![3,-2,1,-5,2,-4]);
        assert_eq!(Solution::rearrange_array(vec![-1,1]), vec![1,-1]);
    }
}
