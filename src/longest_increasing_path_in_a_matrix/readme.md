## Problem

Source: https://leetcode.com/problems/longest-increasing-path-in-a-matrix/

Given an integer matrix, find the length of the longest increasing path.

From each cell, you can either move to four directions: left, right, up or down. You may NOT move diagonally or move outside of the boundary (i.e. wrap-around is not allowed).

Example 1:

Input: nums =
[
  [9,9,4],
  [6,6,8],
  [2,1,1]
]
Output: 4
Explanation: The longest increasing path is [1, 2, 6, 9].

Example 2:

Input: nums =
[
  [3,4,5],
  [3,2,6],
  [2,2,1]
]
Output: 4
Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.

## Solution
- Create a directed acyclic graph (DAG) from the input.
- Order the vertices in topological sort order using depth first search. Remember not to visit any node twice.
- Using the topological sorted vertices, find the longest path: for each outgoing edge from a vertex, if distance of the next vertex can be improved from the current vertex's distance, update the distance of the next vertex.

- Runtime: input is a matrix of size m x n 
  + Create the DAG: O(m x n)
  + Topological sort: O(m x n)
  + Find the longest path: O(m x n) (worst case)