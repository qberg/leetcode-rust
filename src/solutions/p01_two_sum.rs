use std::collections::HashMap;

pub struct Solution {}


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hmap = HashMap::new();
        for (idx,num) in nums.iter().enumerate() {
            match hmap.get(&(target - num)) {
                None => {
                    hmap.insert(num,idx);
                }
                Some(i) => return vec![*i as i32, idx as i32],
            }
        }
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01 () {
        let nums: Vec<i32> = vec![2,3,4];
        let target: i32 = 6;
        assert_eq!(Solution::two_sum(nums,target),vec![0,2]);
    }
}
