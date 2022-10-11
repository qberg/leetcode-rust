> Given a non-empty array `nums` containing only positive integers, find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.

**Solution:**

If the two subsets have equal sum then that value must be half of the total sum of the array.

Let `dp[t]` denote a boolean indicating whether the target `t` is achievable or not. The target in this case will be half of the sum of elements in the given array. A natural checkpoint that arises from this is that the sum of the numbers in the array must be even since all numbers are integers.

`Base Case:` A target `t=0` is always achievable just by not selecting any elements, implying `dp[0] = true`.

We iterate through the given array and check whwther it is possible to form the sum `t` by adding the current number to some previously formed sum. At the end of the iteration, the value of `dp[target]` is returned.

**Complexity Analysis:**

`Time:` Two for loops, one for the numbers in the array and the other for the target sum making the complexity $\mathcal{O}(nums.length \times sum)$.

`Space:` The algorithm uses an 1D dp array of length `sum/2+1` making the complexity $\mathcal{O} (sum)$.

