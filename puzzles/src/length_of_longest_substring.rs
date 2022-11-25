use crate::leetcode::Solution;

/*
 * https://leetcode.com/problems/longest-substring-without-repeating-characters/
 */
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max: i32 = match s.len() {
            0 => 0,
            _ => 1,
        };
        let mut head = 0;
        let mut tail = 1;
        let bytes = s.as_bytes();

        while head < s.len() - 1 && tail < s.len() {
            println!("head = {}, tail = {}", head, tail);
            let jump = if let Some(loc) = &s[head..tail].find(bytes[tail] as char) {
                loc + 1 as usize
            } else {
                0
            };
            head = head + jump;
            tail = tail + 1;
            max = std::cmp::max(max, (tail - head) as i32);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
