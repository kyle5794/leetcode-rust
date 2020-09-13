## Problem

Source: https://leetcode.com/problems/ugly-number-ii/

Write a program to find the n-th ugly number.

Ugly numbers are positive numbers whose prime factors only include 2, 3, 5.

Example:

Input: n = 10
Output: 12
Explanation: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12 is the sequence of the first 10 ugly numbers.

Note:

    1 is typically treated as an ugly number.
    n does not exceed 1690.

## Solution

- Build a sorted vector that only contains ugly number in ascending order. 
- Use 3 pointers to keep track of the number that is built from 2, 3, and 5 respectively
- At each step, find the minimum ugly number that can be build from existing ugly numbers (multiplying 2 or 3 or 5) and 
advance corresponding counters