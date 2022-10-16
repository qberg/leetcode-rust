pub struct Solution {}

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let mut dungeon = dungeon
            .into_iter()
            .collect::<Vec<Vec<i32>>>();
        let n = dungeon.len();
        let m = dungeon[0].len();
        let mut min_health = 1;
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if i + 1 < n && j + 1 < m {
                    min_health = i32::min(dungeon[i+1][j], dungeon[i][j+1]);
                } 
                if i+1 < n && j+1 == m {
                    min_health = dungeon[i+1][j];
                } 
                if j+1 < m && i+1 == n {
                    min_health = dungeon[i][j+1];
                }
                dungeon[i][j] = i32::max(1,-dungeon[i][j] + min_health);
            }
        }
        dungeon[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_174() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ]),
            7
        );
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![0]
            ]),
            1
        );
    }
}
