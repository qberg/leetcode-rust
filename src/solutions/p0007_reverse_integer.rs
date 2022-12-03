pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rev_x = 0;
        while x != 0 {
            match x.checked_mul(10) {
                None => {return 0;},
                Some(tmp) => match tmp.checked_add(x%10){
                    None => {return 0;},
                    Some(pop) => {
                        rev_x = pop;
                    },
                }
            }
            x /= 10;
        }
        rev_x
    }
}
