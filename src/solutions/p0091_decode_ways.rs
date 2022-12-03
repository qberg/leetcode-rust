pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.bytes().map(|b| b - ('0' as u8)).collect::<Vec<u8>>();
        let n = s.len();
        if n==0 {
            return 0;
        }
        let mut dp: Vec<i32> = vec![0;n+1];
        dp[0] = 1;
        dp[1] = match s[0] {
            0 => 0,
            _   => 1,
        };
        for i in 1..n {
            let first = s[i];
            let second = s[i-1] * 10 + s[i];
            if (0..=9).contains(&first) {
                dp[i+1] += dp[i];
            }
            if (10..=26).contains(&second) {
                dp[i+1] += dp[i-1];
            }
        }
        dp[n]
    }
}
