## Problem

Source: https://leetcode.com/problems/longest-consecutive-sequence/

Given an unsorted array of integers, find the length of the longest consecutive elements sequence.

Your algorithm should run in O(n) complexity.

Example:

Input: [100, 4, 200, 1, 3, 2]
Output: 4
Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.


## Solution

1. Count sort (if value range is small)
2. Reduce the problem into a dfs/bfs problem
    - Create a tree: node of value A will have children which has value of (A - 1) and (A + 1)
    - Traverse the tree using dfs/bfs. Mark visited node so that a node will not be visited twice
    - Tree with the most number of nodes is the the longest consecutive sequence