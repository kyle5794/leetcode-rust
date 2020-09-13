## Problem

Source: https://leetcode.com/problems/coin-change/

You are given coins of different denominations and a total amount of money amount. Write a function to compute the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

Example 1:

Input: coins = [1, 2, 5], amount = 11
Output: 3 
Explanation: 11 = 5 + 5 + 1

Example 2:

Input: coins = [2], amount = 3
Output: -1

Note:
You may assume that you have an infinite number of each kind of coin.

## Solution

- Dynamic programming: using coins i1, i2, i3, ..., in to make up amount A
    -> Find the minimum amount of coins to make up the amount of A - i1, A - i2, ..., A - in
- Runtime: input n is the number of different coins, A is the total amount
    + O(n x A)