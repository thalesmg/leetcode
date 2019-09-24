struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        s.chars()
            .enumerate()
            .fold((0, 0, HashMap::new()), |(mut max, mut start, mut seen), (current_index, c)| {
                if let Some(old_index) = seen.insert(c, current_index) {
                    start = std::cmp::max(start, old_index + 1);
                    max = std::cmp::max(max, current_index - start + 1);
                    (max, start, seen)
                } else {
                    max = std::cmp::max(max, current_index - start + 1);
                    (max, start, seen)
                }
            })
            .0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn ex2() {
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn ex4() {
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }

    #[test]
    fn ex5() {
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }

    #[test]
    fn ex6() {
        assert_eq!(Solution::length_of_longest_substring("tmmzuxt".to_string()), 5);
    }
}
