// Time Complexity: O(1)
pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            return 0;
        } else if num%9 == 0 {
            return 9;
        } else {
            return num%9;
        }
    }
}


// Time Complexity: O(n) Solution.
pub struct SolutionLinear {}

impl SolutionLinear {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        while num > 9 {
           num = Self::dig_sum(num)
        }
        num
    }

    fn dig_sum(n: i32) -> i32 {
        n.to_string()
            .as_bytes()
            .into_iter()
            .fold(0,
                |acc, el | acc + (*el as i32 -48)
            )
    }
}
