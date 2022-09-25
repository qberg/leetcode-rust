pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        // Amount of money spent on buying first stock. 
        let mut first_buy:   i32 = 100000;
        // Amount of profit earned by selling. 
        let mut first_sell:  i32 = 0;
        let mut second_buy:  i32 = 100000;
        let mut second_sell: i32 = 0;
        
        for price in prices {
            first_buy   = i32::min(first_buy,price);
            first_sell  = i32::max(first_sell,price-first_buy);
            second_buy  = i32::min(second_buy,price-first_sell);
            second_sell = i32::max(second_sell,price-second_buy);
        }
        second_sell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123 () {
        let prices: Vec<i32> = vec![];
        assert_eq!(Solution::max_profit(prices),0);

        let prices: Vec<i32> = vec![3,3,5,0,0,3,1,4];
        assert_eq!(Solution::max_profit(prices), 6);
    }
}
