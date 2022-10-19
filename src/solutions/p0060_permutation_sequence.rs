pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut res = "".to_string();
        let mut n = n as usize;
        let mut nums = (1..=n).collect::<Vec<usize>>();
        let mut k = k as usize;
        while n > 0 {
            n -= 1;
            let factorial = Solution::factorial(&n);
            let index = (k/factorial) as usize;
            res.push_str(&nums[index].to_string());
            nums.remove(index);
            k %= factorial;
        }
        res
    }

    fn factorial (num: &usize) -> usize {
        match num {
            0 => 1,
            1..=9 => (1..=*num).product(),
            _ => todo!(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60 () {
        assert_eq!(
            Solution::get_permutation(3, 3),
            "213"
        );
    }
}
