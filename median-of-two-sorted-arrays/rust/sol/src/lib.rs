struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let merged = Solution::merge(nums1, nums2);
        let n = merged.len();
        let h = n / 2;

        if n % 2 == 0 {
            (merged[h - 1] + merged[h]) as f64 / 2.0
        } else {
            merged[h] as f64
        }
    }

    fn merge(a1: Vec<i32>, a2: Vec<i32>) -> Vec<i32> {
        let s1 = a1.len();
        let s2 = a2.len();
        let mut out = Vec::with_capacity(s1 + s2);
        let mut i = 0;
        let mut j = 0;

        loop {
            match (a1.get(i), a2.get(j)) {
                (Some(x), Some(y)) => {
                    if x < y {
                        out.push(*x);
                        i += 1;
                    } else {
                        out.push(*y);
                        j += 1;
                    }
                },
                (Some(_), None) => {
                    out.extend_from_slice(&a1[i..]);
                    break
                },
                (None, Some(_)) => {
                    out.extend_from_slice(&a2[j..]);
                    break
                },
                (None, None) => break
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        let a1 = vec![1, 3];
        let a2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(a1, a2), 2.0);
    }

    #[test]
    fn ex2() {
        let a1 = vec![1, 2];
        let a2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(a1, a2), 2.5);
    }
}
