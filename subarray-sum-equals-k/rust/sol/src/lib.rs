struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut empty = HashMap::new();
        empty.insert(0, 1);

        nums
            .iter()
            .fold((0, 0, empty), |(mut count, mut sum, mut acc), x| {
                sum += x;

                if let Some(symmetric_count) = acc.get(&(sum - k)) {
                    count += symmetric_count;
                }

                let curr_count = acc.entry(sum).or_insert(0);
                *curr_count += 1;

                (count, sum, acc)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex() {
        let input = vec![1,1,1];
        assert_eq!(2, Solution::subarray_sum(input, 2));
    }

    #[test]
    fn some_zeros() {
        let input = vec![1,1,1,0,0];
        assert_eq!(4, Solution::subarray_sum(input, 2));
    }

    #[test]
    fn some_negatives() {
        let input = vec![1,1,1,0,0,-1];
        assert_eq!(5, Solution::subarray_sum(input, 2));
    }

    #[test]
    fn some_zero_sum() {
        let input = vec![1,1,1,0,0,-1,-1,1];
        assert_eq!(6, Solution::subarray_sum(input, 2));
    }

    #[test]
    fn more_zero_sum() {
        let input = vec![1,1,1,0,0,-1,-1,-1,1,1];
        assert_eq!(7, Solution::subarray_sum(input, 2));
    }
}
