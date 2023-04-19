pub struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut triangle = triangle
            .into_iter()
            .collect::<Vec<Vec<i32>>>();
        let n = triangle.len();
        if n==1 {
            return triangle[0][0];
        }
        let mut ans = i32::MAX;
        for i in 1..n {
            for j in 0..(i+1) {
                if j==0 {
                    triangle[i][j] = triangle[i-1][j] + triangle[i][j];
                } else if j==i {
                    triangle[i][j] = triangle[i-1][j-1] + triangle[i][j];
                } else {
                    triangle[i][j] = i32::min(triangle[i-1][j], triangle[i-1][j-1]) + triangle[i][j];
                }
            }
        }
        for j in  0..n {
            ans = i32::min(triangle[n-1][j],ans);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_120() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(
            Solution::minimum_total(triangle),
            11
        );
        let triangle = vec![vec![-10]];
        assert_eq!(
            Solution::minimum_total(triangle),
            -10
        );
    }
}
