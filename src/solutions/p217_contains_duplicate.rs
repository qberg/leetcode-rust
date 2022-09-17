pub struct Solution {}


impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        if nums.is_empty() || nums.len() == 1 {
           return false;
        }
        // An overhead for sorting the numbers.
        nums.sort();
        for i in 1..nums.len() {
            if nums[i-1] == nums[i] {
                return true;
            }
        }
        return false;
    }
}






#[cfg(test)]
mod tests {
    // Bringing the parent module into scope.
    use super::*;

    #[test]
    fn test_217 () {
        // An empty string contains no duplicates
        assert_eq!(Solution::contains_duplicate(vec![]), false);
        // A set with one element contains no duplicates
        assert_eq!(Solution::contains_duplicate(vec![0]), false);
        // Given test cases
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true)
    }

}
