/**
 * [102] Binary Tree Level Order Traversal
 *
 * https://leetcode.com/problems/binary-tree-level-order-traversal/
 */
pub struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::traverse(&root, 0, &mut result);
        return result;
    }

    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, output: &mut Vec<Vec<i32>>) {
        match node {
            Some(node) => {
                let node = node.borrow();
                match output.get_mut(level) {
                    Some(o) => o.push(node.val),
                    None => {
                        let mut o = Vec::new();
                        o.push(node.val);
                        output.insert(level, o);
                    }
                };
                Solution::traverse(&node.left, level + 1, output);
                Solution::traverse(&node.right, level + 1, output);
            }
            None => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_102() {
        let v: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(None), v);
        assert_eq!(
            Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![vec![1]]
        );
    }
}
