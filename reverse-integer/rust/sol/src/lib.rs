struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut r;
        let mut acc: i32 = 0;
        let mut x_ = x;

        while x_ != 0 {
            r = x_ % 10;
            x_ /= 10;
            if let Some(acc_) = acc.checked_mul(10) {
                if let Some(acc_) = acc_.checked_add(r) {
                    acc = acc_;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }

        acc
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn ex2() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn ex3() {
        assert_eq!(21, Solution::reverse(120));
    }

    #[test]
    fn max() {
        assert_eq!(0, Solution::reverse(std::i32::MAX));
    }

    #[test]
    fn min() {
        assert_eq!(0, Solution::reverse(std::i32::MIN));
    }

    #[test]
    fn more() {
        assert_eq!(101, Solution::reverse(10100));
    }
}
