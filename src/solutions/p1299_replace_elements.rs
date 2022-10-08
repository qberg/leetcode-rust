pub struct Solution {}

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len(); 
        let mut max: i32 = -1;
        for i in (0..n).rev() {
            let temp = max;
            max = i32::max(temp,arr[i]);
            arr[i] = temp;
        }
        return arr;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1299 () {
        assert_eq!(Solution::replace_elements(vec![400]), vec![-1]);
    }
}
