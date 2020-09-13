## Problems

Source: https://leetcode.com/problems/number-of-islands/

Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
Example 1:

Input: grid = [
  ["1","1","1","1","0"],
  ["1","1","0","1","0"],
  ["1","1","0","0","0"],
  ["0","0","0","0","0"]
]
Output: 1

Example 2:

Input: grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
Output: 3

## Solution

- Connect points in an island using union by rank heuristics (can add path compression too, even though it doesnt do anything for this problem)
- Count the number of nodes which value is as same as its parents value and are not '0';

- Runtime: input is a matrix of size m x n
  + Build disjoint set using union by rank heuristics: O(m x n)
  + Find the root nodes: O(m x n)