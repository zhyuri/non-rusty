/**
 * [150]Evaluate Reverse Polish Notation
 *
 * https://leetcode.com/problems/evaluate-reverse-polish-notation/
 */
pub struct Solution {}

type Op = fn(i32, i32) -> i32;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = Vec::new();
        for token in tokens.iter() {
            if !Solution::is_op(token) {
                stack.push(token.clone());
            } else {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                let result = Solution::calc(left, right, token);
                stack.push(result);
            }
        }
        return stack.first().unwrap().parse::<i32>().unwrap();
    }

    fn is_op(token: &String) -> bool {
        token == "+" || token == "-" || token == "*" || token == "/"
    }

    fn calc(left: String, right: String, op: &String) -> String {
        let op: Op = match op.as_str() {
            "+" => |a: i32, b: i32| -> i32 { a + b },
            "-" => |a: i32, b: i32| -> i32 { a - b },
            "*" => |a: i32, b: i32| -> i32 { a * b },
            "/" => |a: i32, b: i32| -> i32 { a / b },
            _ => panic!("Unknown operator"),
        };
        let left = left.parse::<i32>().unwrap();
        let right = right.parse::<i32>().unwrap();
        return op(left, right).to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_150() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string(),
            ]),
            9
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string(),
            ]),
            6
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string(),
            ]),
            22
        );
    }
}
