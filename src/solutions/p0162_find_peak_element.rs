pub struct Solution {}

impl Solution {
    pub fn find_peak_one_dim (arr: &[i32]) -> i32 {
        let n = arr.len();
        if n==1 {
            return 0;
        } else if n==2 {
            if arr[0] > arr[1] {
                return 0;
            } else {
                return 1;
            }
        } else if arr[n/2 - 1] > arr[n/2] {
            return Solution::find_peak_one_dim(&arr[..n/2]);
        } else if arr[n/2+1] > arr[n/2] {
            return Solution::find_peak_one_dim(&arr[n/2..]);
        } else {
            return (n/2) as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        let arr = vec![1,2,3,1];
        assert_eq!(
            Solution::find_peak_one_dim(&arr),
            2
        );

        let arr = vec![1,2,1,3,5,6,4]; 
        assert_eq!(
            Solution::find_peak_one_dim(&arr),
            5
        );
    }
}
