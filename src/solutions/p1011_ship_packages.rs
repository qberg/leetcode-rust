pub struct Solution {}

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut lower: i32 = 0;
        let mut upper: i32 = 0;
        for w in weights.iter() {
            lower = i32::max(lower,*w);
            upper += w;
        }
        while lower < upper {
            let ship_capacity: i32 = (lower + upper) / 2;
            let needed_days = Self::num_days(&weights, ship_capacity);
            if needed_days <= days {
                upper = ship_capacity;
            } else {
                lower = ship_capacity + 1;
            }
        }
        return lower;
    }

    fn num_days(weights: &[i32], ship_capacity: i32) -> i32 {
        let mut num_days: i32 = 1;
        let mut curr_load: i32 = 0;
        for &w in weights {
            curr_load += w;
            if curr_load > ship_capacity {
                num_days += 1;
                curr_load = w;
            }
        }
        return num_days;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1011 () {
        let weights = vec![1,2,3,4,5,6,7,8,9,10];
        assert_eq!(Solution::ship_within_days(weights,5),15);
    }
}
