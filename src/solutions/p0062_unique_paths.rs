pub struct Solution {}

impl Solution {
    pub fn unique_paths_dp(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![1;n]; m];
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i-1][j] + dp[i][j-1];
            }
        }
        dp[m-1][n-1]
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m==1 || n==1 {
            return 1;
        }
        let mut res = 1;
        let mut m = m-1;
        let mut n = n-1;
        if m < n {
            m = m + n;
            n = m - n;
            m = m - n;
        }
        for i in m+1..=m+n {
            for j in 1..=n {
                res *= i;
                res /= j;
            }
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_62 () {
        const TEST_CASES: [(i32,i32,i32); 2] = [(3,7,28), (3,2,3)];
        for (m,n,res) in TEST_CASES {
            assert_eq!(
                Solution::unique_paths(m, n),
                res
            );
        }
    }
}
