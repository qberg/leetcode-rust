pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut l =  m - 1;
        let mut r =  n - 1;
        while l>=0 && r>=0 {
            if nums1[l as usize] >= nums2[r as usize] {
                nums1[(l+r+1) as usize] = nums1[l as usize];
                l -= 1;
            } else {
                nums1[(l+r+1) as usize] = nums2[r as usize];
                r -= 1;
            }
        }
        while r>=0 {
            nums1[(l+r+1) as usize] = nums2[r as usize];
            r -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_88 () {
        let mut input = vec![1,2,3,0,0,0];
        Solution::merge(&mut input, 3,&mut vec![2,5,6],3);
        assert_eq!(input,vec![1,2,2,3,5,6]);

        // submit 1
        let mut input = vec![1];
        Solution::merge(&mut input, 1,&mut vec![],0);
        assert_eq!(input,vec![1]);
    }
}
