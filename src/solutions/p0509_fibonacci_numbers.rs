pub struct Solution {}

impl Solution {
    pub fn fib_recursive (n: i32) -> i32 {
        if n==0 {
            return 0;
        } else if n==1 {
            return 1;
        } else {
            return Self::fib_recursive(n-1) + Self::fib_recursive(n-1);
        }
    }

    pub fn fib (n:i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0;31];
        dp[0] = 0;
        dp[1] = 1;
        for i in 2..n {
            dp[i] = dp[i-1] + dp[i-2];
        }
        dp[n]
    }
}
