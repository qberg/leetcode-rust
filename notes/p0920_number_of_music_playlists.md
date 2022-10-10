# 920. Number of Music Playlists

> **Question:** Your music player contains n different songs. You want to listen to goal songs (not necessarily different) during your trip. To avoid boredom, you will create a playlist so that:
>
> - Every song is atleast played once.
> - A song can only be played again only if k other songs have been played.
>
> Given n, goal, and k, return the number of possible playlists that you can create. Since the answer can be very large, return it modulo $10^9 + 7$.

**Solution**

Let `dp[s]` denote the number of playlist consisiting of `s` unique songs. Initially it is filled with zeros and the number of songs in our playlist,`l` is set to 1.

`Base cases:`
- `s=0` implies `dp[0]=0` as there can be no playlist with zero songs.
- `s=1` implies `dp[1]=n` as there are `n` different songs.

Now we increase the number of songs in our playlist. The following cases arise when we add the next song to the playlist,

`Case 1:` We add a song that has not yet been played. There are `n - (s-1)` such songs to chose from.

`Case 2:` We add a song that was listened before `k` songs. There are `s-k` such songs to choose from.

The recursion can be defined as,

`dp[s] = dp[s-1] * (n-s+1) + dp[s] * (s-k)`

The answer expected is `dp[n]` in the iteration where `l=goal`.

**Complexity analysis**

`Time:` We need to iterate over all values of length of playlists ranging from `0..goal` and for each such length we need to loop over the number of unique songs that lie in the range `1..min(n,l)` making the worst case time complexity $\mathcal{O}(n \times goal)$.

`Space:` The algorithm uses an auxillary array of size `goal+1` making the space complexity $\mathcal{O}(goal)$.
