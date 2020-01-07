/**
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/
 */
pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for cha in s.chars() {
            match stack.last() {
                None => stack.push(cha),
                Some(last) => {
                    if (*last == '(' && cha == ')')
                        || (*last == '[' && cha == ']')
                        || (*last == '{' && cha == '}')
                    {
                        stack.pop();
                        continue;
                    } else {
                        stack.push(cha);
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::is_valid("()()(){}".to_string()), true);
        assert_eq!(Solution::is_valid("()(]{}".to_string()), false);
        assert_eq!(Solution::is_valid("()(]){}".to_string()), false);
        assert_eq!(Solution::is_valid("(])[{}".to_string()), false);
    }
}
