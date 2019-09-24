struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        s.chars()
            .enumerate()
            .fold((0, 0, HashMap::new()), |(mut max, mut current, mut seen), (i, c)| {
                if let Some(mut old_index) = seen.insert(c, i) {
                    current = (i - old_index) as i32;
                    seen.retain(|_, j| j > &mut old_index);
                    (max, current, seen)
                } else {
                    current += 1;
                    if current > max {
                        max = current;
                    }
                    (max, current, seen)
                }
            })
            .0
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
