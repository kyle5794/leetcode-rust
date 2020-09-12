## Problem
Given a string s, find the length of the longest substring without repeating characters.

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

Example 4:

Input: s = ""
Output: 0

 

Constraints:

    0 <= s.length <= 5 * 104
    s consists of English letters, digits, symbols and spaces.

## Analysis

- Scan the string from left to right and maintain two pointers `i` and `j` as the `[start, end]` of the current substring
- If char at `j+1` is an duplicate with char at `j'` (`j'` ∈ `[i, j]`). Update `i` = `j' + 1` 
- Runtime: O(n) n is the length of the string
- Note: using array of size 128 (for ASCII characters) to store position of a character runs faster than using a hash map
as indexing primitive array is faster than hash map lookup. However, the memory consumption would increase