# Backtracking Algorithm

> Backtracking is an algorithm for finding all solutions by exploring all potential candidates. If the solution candidate turns to be not a solution (or at least not the last one), backtracking algorithm discards it by making some changes on the previous step, i.e. backtracks and then try again.

**[78.Subsets](https://leetcode.com/problems/subsets/)**

Given the definition, the problem can also be interpreted as finding the power set from a sequence.

One could loop over the length of combination, rather than the candidate numbers, and generate all combinations for a given length with the help of backtracking technique.

![](https://github.com/qberg/leetcode-rust/blob/master/notes/images/p78_1.png)

![](https://github.com/qberg/leetcode-rust/blob/master/notes/images/p78_2.png)

**Algorithm**

We define a backtrack function named `backtrack(first, curr)` which takes the index of first element to add and a current combination as arguments.

* If the current combination is done, we add the combination to the final output.

* Otherwise, we iterate over the indexes i from first to the length of the entire sequence n.

    * Add integer `nums[i]` into the current combination `curr`.

    * Proceed to add more integers into the combination: `backtrack(i + 1, curr)`.

    * Backtrack by removing `nums[i]` from `curr`.
