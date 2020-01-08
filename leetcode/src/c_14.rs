/**
 * [14] Longest Common Prefix
 *
 * https://leetcode.com/problems/longest-common-prefix/
 */
pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_len = strs.iter().map(|str| str.len()).min().unwrap_or_default();
        let mut i = 0;
        while i < min_len {
            let mut arr: Vec<char> = strs.iter().map(|str| str.chars().nth(i).unwrap()).collect();
            arr.dedup();
            if arr.len() > 1 {
                break;
            }
            i += 1;
        }
        if i > 0 {
            strs[0].get(0..i).unwrap().to_string()
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl"
        );
        assert_eq!(Solution::longest_common_prefix(vec!["a".to_string()]), "a");
        assert_eq!(
            Solution::longest_common_prefix(vec!["c".to_string(), "c".to_string()]),
            "c"
        );
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}
