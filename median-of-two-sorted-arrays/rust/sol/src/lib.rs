struct Solution;

/// https://medium.com/@hazemu/finding-the-median-of-2-sorted-arrays-in-logarithmic-time-1d3f2ecbeb46
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let s1 = nums1.len();
        let s2 = nums2.len();
        let is_even = (s1 + s2) % 2 == 0;
        let nhalf = (s1 + s2 + 1) / 2;

        // len(a) <= len(b)
        let (a, b) = if s1 < s2 {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        // if one is empty, done.
        if a.len() == 0 {
            return Solution::median_for_one(b)
        }

        let (mut na_min, mut na_max) = (0, a.len());

        loop {
            let na = (na_min + na_max) / 2;
            let nb = nhalf - na;

            if na > 0 && a[na - 1] > b[nb] {
                // a is contributing at least 1 element, so b is
                // cannot be contributing all its elements
                na_max = na - 1;
            } else if na < a.len() && a[na] < b[nb - 1] {
                // there is at least 1 element from a that is not
                // being contributed, which means b is contributing at
                // least 1 element
                na_min = na + 1;
            } else {
                // the solution in either in a[0..na] or in b[0..nb],
                // or it is the mean of values from both.

                // maximum value of left half
                let candidate_l = if na == 0 {
                    // if a is not participating, then it is all b
                    b[nb - 1]
                } else if nb == 0 {
                    // if b is not participating, then it is all a
                    a[na - 1]
                } else {
                    std::cmp::max(a[na - 1], b[nb - 1])
                };

                if is_even {
                    // the smallest value >= candidate_l
                    let candidate_r = if na == a.len() {
                        // if a is all in the left side, nb < b.len()
                        b[nb]
                    } else if nb == b.len() {
                        // if b is all in the left side, na < a.len()
                        a[na]
                    } else {
                        std::cmp::min(a[na], b[nb])
                    };

                    return (candidate_l + candidate_r) as f64 / 2.0
                } else {
                    return candidate_l as f64
                }
            }
        }
    }

    fn median_for_one(a: Vec<i32>) -> f64 {
        let nhalf = a.len() / 2;
        if a.len() & 1 == 0 {
            (a[nhalf] + a[nhalf - 1]) as f64 / 2.0
        } else {
            a[nhalf] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        let a = vec![1, 3];
        let b = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 2.0);
    }

    #[test]
    fn ex2() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 2.5);
    }

    #[test]
    fn ex3_odd() {
        let a = vec![4, 20, 32, 50, 55, 61];
        let b = vec![1, 15, 22, 30, 70];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 30.0);
    }

    #[test]
    fn ex3_even() {
        let a = vec![4, 20, 32, 50, 55];
        let b = vec![1, 15, 22, 30, 70];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 26.0);
    }

    #[test]
    fn ex4() {
        let a = vec![1];
        let b = vec![1];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 1.0);
    }

    #[test]
    fn ex5() {
        let a = vec![1, 2];
        let b = vec![1, 2];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 1.5);
    }

    #[test]
    fn ex5a() {
        let a = vec![1, 2];
        let b = vec![1, 1];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 1.0);
    }

    #[test]
    fn ex5b() {
        let a = vec![1, 1];
        let b = vec![1, 2];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 1.0);
    }

    #[test]
    fn ex6a() {
        let a = vec![1];
        let b = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 1.5);
    }

    #[test]
    fn ex6b() {
        let a = vec![2];
        let b = vec![1];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 1.5);
    }

    #[test]
    fn ex7() {
        let a = vec![1];
        let b = vec![2, 3];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 2.0);
    }

    #[test]
    fn a_empty_even() {
        let a = vec![];
        let b = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 2.5);
    }

    #[test]
    fn a_empty_odd() {
        let a = vec![];
        let b = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 3.0);
    }

    #[test]
    fn b_empty_even() {
        let a = vec![1, 2, 3, 4];
        let b = vec![];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 2.5);
    }

    #[test]
    fn b_empty_odd() {
        let a = vec![1, 2, 3, 4, 5];
        let b = vec![];
        assert_eq!(Solution::find_median_sorted_arrays(a, b), 3.0);
    }
}
