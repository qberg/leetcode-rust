pub struct Solution {}

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len();
        let m = mat[0].len();
        let mut low = 0;
        let mut high = n-1;
        let mut max_index: usize = 0;
        while low < high {
            let mid = (high - low)/2 + low;
            max_index = Solution::max(&mat[mid]);
            if mid + 1 < n && mat[mid+1][max_index] > mat[mid][max_index] {
                low = mid + 1 ;
            } else {
                high = mid;
            }
        }
        vec![low as i32,max_index as i32]
    }

    fn max(arr: &[i32]) -> usize {
        let mut max = arr[0];
        let mut max_index = 0;
        for (index,&x) in arr
            .iter()
            .enumerate() {
                if x > max {
                    max = x;
                    max_index = index;
                }
            }
        max_index
    }
}

