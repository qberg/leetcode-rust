pub struct Solution {}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut queens_rem = n as usize;
        let mut board = vec![vec!['.';n as usize]; n as usize];
        let mut res = Vec::new();
        Solution::place_queens(&mut board, n as usize, queens_rem, &mut res);
        res
    }

    fn place_queens(
        board: &mut Vec<Vec<char>>,
        n: usize,
        queens_rem: usize,
        res: &mut Vec<Vec<String>>
    ) {
        if queens_rem == 0 {
            res.push(board
                .iter()
                .map(|vec| vec.iter().collect())
                .collect()
            );
            return;
        } 
        for col in 0..n {
            let row = n - queens_rem;
            if Solution::is_valid(&board, n, row, col) {
                board[row][col] = 'Q';
                Solution::place_queens(&mut board, n, queens_rem-1, &mut res);
                board[row][col] = '.';
            }
        }
    }

    fn is_valid(
        board: &Vec<Vec<char>>,
        n: usize,
        row: usize,
        col: usize,
    ) -> bool {
        for r in 0..row {
            if board[r][col] == 'Q' {
                return false;
            }
        }
        let mut r = row as i32 - 1;
        let mut c = col as i32 - 1;
        while r>=0 && c>=0 {
            if board[r as usize][c as usize] == 'Q' {
                return false;
            } else {
                r -= 1;
                c -= 1;
            }
        }
        let mut r = row as i32 - 1;
        let mut c = col as i32 + 1;
        while r>=0 && c < n as i32{
            if board[r as usize][c as usize] == 'Q' {
                return false;
            } else {
                r -= 1;
                c += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(
            Solution::solve_n_queens(1),
            vec![vec!["Q"]]
            );
    }
}
