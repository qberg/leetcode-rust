use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        if croak_of_frogs.len() % 5 != 0 {
            return -1;
        }
        let mut res: i32 = 0;
        let mut machines: HashMap<char, usize> = "croak".chars().map(|c| (c, 0_usize)).collect();
        for ch in croak_of_frogs.chars() {
            if ch == 'c' {
                *machines.get_mut(&'c').unwrap() += 1;  
            } else if ch == 'r' {
                if machines[&'c'] > 0 {
                    *machines.get_mut(&'c').unwrap() -= 1;
                    *machines.get_mut(&'r').unwrap() += 1;
                } else {
                    return -1;
                }
            } else if ch == 'o' {
                if machines[&'r'] > 0 {
                    *machines.get_mut(&'r').unwrap() -= 1;
                    *machines.get_mut(&'o').unwrap() += 1;
                } else {
                    return -1;
                }
            } else if ch == 'a' {
                if machines[&'o'] > 0 {
                    *machines.get_mut(&'o').unwrap() -= 1;
                    *machines.get_mut(&'a').unwrap() += 1;
                } else {
                    return -1;
                }
            } else if ch == 'k' {
                if machines[&'a'] > 0 {
                    *machines.get_mut(&'a').unwrap() -= 1;
                    *machines.get_mut(&'k').unwrap() += 1;
                } else {
                    return -1;
                }
            }
            res = i32::max(res,Solution::count_machines(&machines) as i32);
        } 
        if Solution::is_valid(&machines) {
            return res;
        } else {
            return -1;
        }
    }

    fn count_machines (machines: &HashMap<char,usize>) -> usize {
        let mut sum: usize = 0;
        for c in &['c','r','o','a'] {
            sum += machines[c];
        }
        return sum;
    }

    fn is_valid (machines: &HashMap<char,usize>) -> bool {
        for c in &['c','r','o','a'] {
            if machines[c] != 0 {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1011 () {
        assert_eq!(Solution::min_number_of_frogs("croakcroak".to_string()),1);
    }
}
