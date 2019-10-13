struct Solution;

type Palindrome = (usize, usize);

impl Solution {
    pub fn longest_palindrome(full: String) -> String {
        let mut palindromes = Vec::new();
        let mut farthest = 0;

        for (i, _) in full.chars().enumerate() {
            if i < farthest {
                continue
            };

            let mut current = (i, i);
            while let Some(bigger) = Solution::try_increase_palindrome(&full, current) {
                let (m, n) = bigger;
                farthest = std::cmp::max(n, farthest);

                if bigger == current {
                    break
                } else {
                    current = bigger;
                }
            }
            palindromes.push(current);
        }

        dbg!(&palindromes);

        if let Some((s, e)) = palindromes.into_iter().max_by_key(|(s, e)| e - s) {
            full[s..=e].to_string()
        } else {
            String::new()
        }
    }

    pub fn try_increase_palindrome(full: &str, palindrome: Palindrome) -> Option<Palindrome> {
        let (start, end) = palindrome;
        let full_str: String = full.to_string();
        let len = full.len();
        let next_end = if end + 1 >= len { end } else { end + 1 };
        let next_start = if start == 0 { 0 } else { start - 1 };

        if let Some(slice) = full_str.get(next_start..=next_end) {
            if Solution::is_palindrome(slice) {
                return Some((next_start, next_end));
            }
        };

        if let Some(slice) = full_str.get(start..=next_end) {
            if Solution::is_palindrome(slice) {
                return Some((start, next_end));
            }
        };

        None
    }

    pub fn is_palindrome(s: &str) -> bool {
        let reverse = s.chars().rev();

        for (a, b) in s.to_string().chars().zip(reverse) {
            if a != b {
                return false;
            }
        }

        true
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

    #[test]
    fn is_palindrome() {
        assert!(Solution::is_palindrome(""));
        assert!(Solution::is_palindrome("a"));
        assert!(Solution::is_palindrome("aa"));
        assert!(!Solution::is_palindrome("ba"));
        assert!(Solution::is_palindrome("bab"));
        assert!(Solution::is_palindrome("baab"));
    }

    #[test]
    fn try_increase_palindrome() {
        let full = "babad";
        assert_eq!(Solution::try_increase_palindrome(full, (0, 0)), None);
        assert_eq!(
            Solution::try_increase_palindrome(full, (0, 1)),
            Some((0, 2))
        );
        assert_eq!(
            Solution::try_increase_palindrome(full, (1, 1)),
            Some((0, 2))
        );
        assert_eq!(Solution::try_increase_palindrome(full, (0, 2)), None);
    }
}
