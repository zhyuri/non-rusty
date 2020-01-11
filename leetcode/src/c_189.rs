/**
 * [189] Rotate Array
 *
 * https://leetcode.com/problems/rotate-array/
 */
pub struct Solution {}

use std::collections::LinkedList;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if k <= 0 {
            return;
        }
        let mut k = k as usize;
        if k > nums.len() {
            k = k % nums.len();
        }
        let mut buffer: LinkedList<i32> = LinkedList::new();
        for i in (nums.len() - k)..nums.len() {
            buffer.push_back(nums[i]);
        }
        for i in 0..nums.len() {
            buffer.push_back(nums[i]);
            nums[i] = buffer.pop_front().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_189() {
        let mut under_test = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut under_test, 3);
        assert_eq!(under_test, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut under_test = vec![1, 2];
        Solution::rotate(&mut under_test, 3);
        assert_eq!(under_test, vec![2, 1]);
    }
}
