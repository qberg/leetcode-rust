pub struct Solution {} 

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res =vec![];
        let n = nums.len();
        if n < 4 {
            return res;
        }
        let mut nums = nums;
        nums.sort_unstable();
        for i in 0..n-3 {
            if i>0 && nums[i] == nums[i-1] {
                continue;
            }
            Self::three_sum (nums[i], &nums[i+1..n], target, &mut res);
        }
        res
    }

    fn three_sum (
        first: i32,
        nums: &[i32],
        target: i32,
        res: &mut Vec<Vec<i32>>
    ) {
        let target = target - first;
        for i in 0..nums.len() -2 {
            if i > 0  && nums[i] == nums[i-1] {
                continue;
            }
            let (mut l, mut r) = (i+1, nums.len() -1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                match sum.cmp(&target) {
                    std::cmp::Ordering::Equal => {
                        res.push(
                            vec![first, nums[i], nums[l], nums[r]]
                        );
                        while l<r && nums[l] == nums[l+1] {
                            l+=1;
                        }
                        l += 1;
                        while l<r && nums[r] == nums[r-1] {
                            r-=1;
                        }
                        r -= 1;
                    },
                    std::cmp::Ordering::Less => {
                        l += 1;
                    },
                    std::cmp::Ordering::Greater => {
                        r -= 1;
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18 () {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![
                vec![-2, -1, 1, 2],
                vec![-2,  0, 0, 2],
                vec![-1,  0, 0, 1],
            ]
        );

        assert_eq!(
            Solution::four_sum(vec![-1,-5,-5,-3,2,5,0,4], -7),
            vec![
                vec![-5,-5,-1,4],
                vec![-5,-3,-1,2],
            ]
        );

        assert_eq!(
            Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -2222219919),
            vec![vec![]]
        );
    }
}
