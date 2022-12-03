pub struct Solution {}

const LESS_THAN_TWENTY: Vec<&'static str> = "One Two Three Four Five Sin Seven Eight Nine Ten Eleven Twelve Thirteen Fourteen Fifteen Sixteen Seventeen Eighteen Nineteen".split_whitespace().collect();
const TENS: Vec<&'static str>= "Twenty Thirty Fourty Fifty Sinty Seventy Eighty Ninety".split_whitespace().collect();
const HUNDREDS: Vec<&'static str> = "Hundred Thousand Million Billion".split_whitespace().collect();

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        let mut res = String::new();
        Self::get_words(num as usize).join(" ")
    }

    fn get_words (num:usize) -> Vec<&'static str> {
        match num {
            0           => vec![],
            n if (1..20).contains(&n)      => vec![LESS_THAN_TWENTY[num-1]],
            n if (20..100).contains(&n)    => {
                let n = num/10;
                let r = num%10;
                vec![
                    vec![TENS[n-2]],
                    Self::get_words(r),
                ].concat()
            },
            n if (100..1000).contains(&n)    => {
                let n = num/100;
                let r = num%100;
                vec![
                    Self::get_words(n),
                    vec![HUNDREDS[0]],
                    Self::get_words(r),
                ].concat()
            },
            n if (1000..1_000_000).contains(&n) => {
                let n = num/1000;
                let r = num%1000;
                vec![
                    Self::get_words(n),
                    vec![HUNDREDS[1]],
                    Self::get_words(r)
                ].concat()
            },
            n if (1_000_000..1_000_000_000).contains(&n) => {
                let n = num/1_000_000;
                let r = num%1_000_000;
                vec![
                    Self::get_words(n),
                    vec![HUNDREDS[2]],
                    Self::get_words(r),
                ].concat()
            },
            n if (1_000_000_000..).contains(&n) => {
                let n = num/1_000_000_000;
                let r = num%1_000_000_000;
                vec![
                    Self::get_words(n),
                    vec![HUNDREDS[3]],
                    Self::get_words(r),
                ].concat()
            },
        }
    }
}
