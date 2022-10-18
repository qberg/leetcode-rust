#[derive(PartialEq,Clone,Copy)]
pub enum State {
    Start,
    FoundSign,
    FoundDigit,
    FoundDot,
    FoundExp,
    FoundSignAfterExp,
    FoundDigitAfterExp,
    FoundDotAfterDigit,
    Dead,
}

pub enum Action {
    Sign,
    Digit,
    Dot,
    Exp,
    Other,
}

use State::*;
use Action::*;
pub struct Solution {} 

impl Solution {
    pub fn is_number(s: String) -> bool {
        let terminal_states = vec![
            FoundDigit,
            FoundDotAfterDigit,
            FoundDigitAfterExp
        ];
        let mut state = Start;
        for c in s.chars() {
            if state == Dead {
                return false;
            }
            let action = Solution::get_action(c);
            state = Solution::transition(&state, &action);
        }
        if terminal_states.iter().any(|&s| s == state) {
            return true;
        } else {
            return false;
        }
    }

    fn get_action (c: char) -> Action {
        match c {
            '+' | '-' => Sign,
            '0'..='9' => Digit,
            '.'       => Dot,
            'e' | 'E' => Exp,
            _         => Other,
        }
    }

    fn transition (state: &State, action: &Action) -> State {
        match (state, action) {
            (_, Other) => Dead,
            (Dead,_)   => Dead,
            (Start,Digit) => FoundDigit,
            (Start,Sign)  => FoundSign,
            (Start,Dot)   => FoundDot,
            (Start,Exp)   => Dead,
            (FoundDigit,Digit) => FoundDigit,
            (FoundDigit,Dot)   => FoundDotAfterDigit,
            (FoundDigit,Exp)   => FoundExp,
            (FoundDigit,Sign)  => Dead,
            (FoundSign,Digit) => FoundDigit,
            (FoundSign,Dot)   => FoundDot,
            (FoundSign,_)    => Dead,
            (FoundDot,Digit) => FoundDotAfterDigit,
            (FoundDot,_)     => Dead,
            (FoundExp,Sign)  => FoundSignAfterExp,
            (FoundExp,Digit) => FoundDigitAfterExp,
            (FoundExp,_)     => Dead,
            (FoundDotAfterDigit,Digit) => FoundDotAfterDigit,
            (FoundDotAfterDigit,Exp)   => FoundExp,
            (FoundDotAfterDigit,_)     => Dead,
            (FoundSignAfterExp,Digit)  => FoundDigitAfterExp,
            (FoundSignAfterExp,_)      => Dead,
            (FoundDigitAfterExp,Digit) => FoundDigitAfterExp,
            (FoundDigitAfterExp,_)     => Dead,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_65 () {
        let s = "0".to_string();
        assert_eq!(
            Solution::is_number(s),
            true
        );
        
        let s = ".".to_string();
        assert_eq!(
            Solution::is_number(s),
            false
        );

        let s = "+.8".to_string();
        assert_eq!(
            Solution::is_number(s),
            true
        );
    }
}
