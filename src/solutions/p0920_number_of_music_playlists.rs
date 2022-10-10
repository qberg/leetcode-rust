pub struct Solution {}

impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        const MODULO: u128 = 1000000007;
        let mut num_us: usize;
        let mut dp: Vec<u128> = vec![0;n as usize + 1];
        dp[1] = n as u128;
        for l in 2..goal+1 {
            num_us = i32::min(n, l) as usize;
            for s in (1..num_us + 1).rev() {
                dp[s] = (dp[s-1] * (n-s as i32+1) as u128 + dp[s] * i32::max(s as i32 - k, 0) as u128) % MODULO;
            }
        }
        dp[n as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_920 () {
        assert_eq!(Solution::num_music_playlists(3,3,1), 6);
        assert_eq!(Solution::num_music_playlists(2,3,0), 6);
        assert_eq!(Solution::num_music_playlists(2,3,1), 2);
    }
}
