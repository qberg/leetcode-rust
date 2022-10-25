// Solutions using iterators...
pub struct Solution {}

pub struct VecsIter {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
}

impl VecsIter {
    fn len(&self) -> usize {
        self.vec1.len() + self.vec2.len()
    }
}


impl Iterator for VecsIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.vec1.last(), self.vec2.last()) {
            (Some(n1), Some(n2)) if n1 >= n2 => self.vec1.pop(),
            (Some(_), Some(_))               => self.vec2.pop(),
            (Some(_), _)                     => self.vec1.pop(),
            (_, Some(_))                     => self.vec2.pop(),
            _                                => None,
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut vecs_iter = VecsIter {
            vec1: nums1,
            vec2: nums2,
        };
        let length = vecs_iter.len();
        let middle = length / 2;
        if 0 == length % 2 {
            let mut halves = vecs_iter.skip(middle - 1);
            (halves.next().unwrap() as f64 + halves.next().unwrap() as f64) / 2.0
        } else {
            vecs_iter.nth(middle).unwrap() as f64
        }
    }
}


// Solution that creates a new vector...
pub struct SolutionOne {}


impl SolutionOne {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums: Vec<i32> = vec![];
        let (mut p1, mut p2) = (0,0);
        while p1 < nums1.len() && p2 < nums2.len() {
            let n1 = nums1[p1];
            let n2 = nums2[p2];
            match n1.cmp(&n2) {
                std::cmp::Ordering::Less    => {
                    nums.push(n1);
                    p1 += 1;
                },
                std::cmp::Ordering::Greater => {
                    nums.push(n2);
                    p2 += 1;
                },
                std::cmp::Ordering::Equal  => {
                    nums.push(n1);
                    nums.push(n2);
                    p1 += 1;
                    p2 += 1;
                },
            };
        }
        if p1 < nums1.len() {
            for &n in &nums1[p1..] {
                nums.push(n);
            }
        }
        if p2 < nums2.len() {
            for &n in &nums2[p2..] {
                nums.push(n);
            }
        }
        let l = nums.len();
        if l % 2 == 0 {
            let l  = nums.len();
            let ans = (nums[l/2] + nums[l/2 -1]) as f64;
            return ans/2_f64;
        } else {
            return nums[l/2] as f64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: implementation
    #[test]
    #[ignore]
    fn test_4() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
