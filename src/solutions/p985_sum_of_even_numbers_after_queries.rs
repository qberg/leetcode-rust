pub struct Solution {}

impl Solution{
    fn sum_even_after_queries(mut nums: Vec<i32>, queries:Vec<Vec<i32>>) -> Vec<i32> {
         let mut sum: i32 = nums.iter().filter(|&a| a%2==0).sum();
         let mut ans: Vec<i32> = vec![];
         for query in queries {
             let val = query[0];
             let index = query[1] as usize;
             if nums[index] % 2 == 0{
                 sum -= nums[index];
             }
             nums[index] += val;
             if nums[index] %2 == 0 {
                 sum += nums[index];
             }
             ans.push(sum);
         }    
         ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_985 () {
        let nums = vec![1,2,3,4];
        let queries = vec![vec![1,0],vec![-3,1],vec![-4,0],vec![2,3]];
        assert_eq!(Solution::sum_even_after_queries(nums, queries), vec![8,6,2,4]);
    }

}
