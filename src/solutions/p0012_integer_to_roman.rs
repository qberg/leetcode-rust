pub struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num as usize;
        let mut res = String::new();
        let m = vec!["","M","MM","MMM"];
        let c = vec!["","C","CC","CCC","CD","D","DC","DCC","DCCC","CM"];
        let x = vec!["","X","XX","XXX","XL","L","LX","LXX","LXXX","XC"];
        let i = vec!["","I","II","III","IV","V","VI","VII","VIII","IX"];
        let thousands = num/1000;
        res.push_str(m[thousands]);
        let hundreds  = (num%1000)/100;
        res.push_str(c[hundreds]);
        let tens = (num%100)/10;
        res.push_str(x[tens]);
        let ones = num%10;
        res.push_str(i[ones]);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
