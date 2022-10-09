pub struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums
            .into_iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| {
            let str1 = format!("{}{}",b,a);
            let str2 = format!("{}{}",a,b);
            str1.cmp(&str2)
        });
        if nums[0] == "0" {
            return "0".to_owned();
        } else {
            return nums.join("");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_179() {
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_owned()
        );
        assert_eq!(
            Solution::largest_number(vec![2, 10]), 
            "210".to_owned()
        );
    }
}
