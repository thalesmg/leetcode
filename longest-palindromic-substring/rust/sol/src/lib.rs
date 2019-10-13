struct Solution;

impl Solution {
    pub fn longest_palindrome(full: String) -> String {
        let len = full.len();
        let mut start = 0;
        let mut end = 0;

        // centers between or at chars
        for i in 0..(len as i32) {
            // at letter
            let len1 = Solution::expand_around_center(&full, i, i);
            // between letters
            let len2 = Solution::expand_around_center(&full, i, i + 1);
            let max_len = std::cmp::max(len1, len2);

            if (max_len > end - start) {
                start = i - (max_len as i32 - 1) / 2;
                end = i + (max_len as i32) / 2;
            }
        }

        full.get((start as usize)..=(end as usize)).unwrap_or("").to_string()
    }

    fn expand_around_center(full: &str, mut start: i32, mut end: i32) -> i32 {
        let len = full.len();

        while start >= 0 && end < len as i32 {
            let s = start as usize;
            let e = end as usize;

            match (full.get(s..=s), full.get(e..=e)) {
                (Some(a), Some(b))
                    if a == b => {
                    start -= 1;
                    end += 1;
                },
                _ => break,
            }
        }

        end - start - 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn ex1() {
        let input = "babad".to_string();
        let expected: HashSet<String> =
            HashSet::from_iter(vec!["bab", "aba"].into_iter().map(|r| r.to_string()));
        let mut result = HashSet::new();
        result.insert(Solution::longest_palindrome(input));
        assert!(expected.is_superset(&result));
    }

    #[test]
    fn ex2() {
        let input = "cbbd".to_string();
        let expected: HashSet<String> =
            HashSet::from_iter(vec!["bb"].into_iter().map(|r| r.to_string()));
        let mut result = HashSet::new();
        result.insert(Solution::longest_palindrome(input));
        assert!(expected.is_superset(&result));
    }

    #[test]
    fn ex3() {
        let input = "".to_string();
        assert_eq!("".to_string(), Solution::longest_palindrome(input));
    }

    #[test]
    fn ex4() {
        let input = "abc".to_string();
        let expected: HashSet<String> =
            HashSet::from_iter(vec!["a", "b", "c"].into_iter().map(|r| r.to_string()));
        let mut result = HashSet::new();
        result.insert(Solution::longest_palindrome(input));
        assert!(expected.is_superset(&result));
    }

    #[test]
    fn ex5() {
        let input = "dabab".to_string();
        let expected: HashSet<String> =
            HashSet::from_iter(vec!["bab", "aba"].into_iter().map(|r| r.to_string()));
        let mut result = HashSet::new();
        result.insert(Solution::longest_palindrome(input));
        assert!(expected.is_superset(&result));
    }

    #[test]
    fn ex6() {
        let input = "abcdefedcbz".to_string();
        let expected: HashSet<String> =
            HashSet::from_iter(vec!["bcdefedcb"].into_iter().map(|r| r.to_string()));
        let mut result = HashSet::new();
        result.insert(Solution::longest_palindrome(input));
        assert!(expected.is_superset(&result));
    }

    #[test]
    fn ex7() {
        let input = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string();
        let expected = input.clone();
        assert_eq!(expected, Solution::longest_palindrome(input));
    }

    #[test]
    fn ex8() {
        let input = "ababababababa".to_string();
        let expected = input.clone();
        assert_eq!(expected, Solution::longest_palindrome(input));
    }
}
