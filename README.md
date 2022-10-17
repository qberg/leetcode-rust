<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/qberg/leetcode-rust">
    <img src="./assets/images/logo.png" alt="Logo" width="680" height="256">
  </a>
</p>

[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=102)](https://github.com/ellerbrock/open-source-badge/)
![](https://img.shields.io/badge/%3E-leetcode-green.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)&nbsp;
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)&nbsp;
![Update](https://img.shields.io/badge/update-daily-green.svg)&nbsp;

A repo for all the leetcode problems that I have solved using rust-lang. Note that the solutions might not be as rust idiomatic as they could be for I have solved them while beginning to explore rust world. 

> *Polynomial time is great. OK? Exponential time is bad. And infinite time gets you fired.* 
>                                                                                            
> *-- Prof. Srini Devadas*

## Stats 

![Leetcode Stats](https://leetcard.jacoblin.cool/qberg?theme=dark)

## Problems
### Easy 

|  #  |                Title                            | Solution |         Quick notes          | Analysis | Time | Space |
| :-: | :---------------------------------------------: | :------: | :--------------------------: | :------: | :--: | :--: |
| 1832 | [Check if the Sentence Is Pangram](https://leetcode.com/problems/check-if-the-sentence-is-pangram/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p1832_check_if_pangram.rs) | Use a `Hash Set` to keep track of the characters seen.| |`O(n)` | `O(1)` |


### Medium

|  #  |                Title                            | Solution |         Quick notes          | Analysis | Time | Space  
| :-: | :---------------------------------------------: | :------: | :--------------------------: | :------: | :--: | :--: |
| 64  | [Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0064_minimum_path_sum.rs) | Use `dynamic programming` to find the minimum path among all possible paths before the current state and then add the value for the current state. | | Medium | | 
| 120 | [Triangle](https://leetcode.com/problems/triangle/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0120_triangle.rs) | `Dynamic programming` pattern of finding the minimum path similar to p64. | | Medium | |
| 162 | [Find Peak Element](https://leetcode.com/problems/find-peak-element/https://leetcode.com/problems/find-peak-element/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0162_find_peak_element.rs) | Use `binary search` with a left and right index. | | Medium || 
| 416 | [Partition Equal Subset Sum](https://leetcode.com/problems/partition-equal-subset-sum/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0416_partition_equal_subset_sum.rs) | `Dynamic Programming` | [:memo:](https://github.com/qberg/leetcode-rust/blob/master/notes/p0416_partition_equal_subset_sum.md) | Medium | |

### Hard

|  #  |                Title                            | Solution |         Quick notes          | Analysis | Time | Space |
| :-: | :---------------------------------------------: | :------: | :--------------------------: | :------: | :--: | :--: |
| 51  | [N-Queens - II](https://leetcode.com/problems/n-queens-ii/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0052_total_n_queens.rs) | `Backtracking` | |  |  |
| 52  | [N-Queens](https://leetcode.com/problems/n-queens/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0051_solve_n_queens.rs) | `Backtracking` | |  |  |
| 174 | [Dungeon Game](https://leetcode.com/problems/dungeon-game/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0174_dungeon_game.rs) | `Dynamic programming` pattern similar to p64. | | Hard | |
| 920 | [Number of Music Playlists](https://leetcode.com/problems/number-of-music-playlists/) | [Rust](https://github.com/qberg/leetcode-rust/blob/master/src/solutions/p0920_number_of_music_playlists.rs) | `Dynamic Programming` | [:memo:](https://github.com/qberg/leetcode-rust/blob/master/notes/p0920_number_of_music_playlists.md) | Hard | |












