struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let sums =
            nums
            .iter()
            .enumerate()
            .fold((0, HashMap::new()), |(mut sum, mut acc), (i, x)| {
                sum += x;
                let indices = acc.entry(sum).or_insert(Vec::new());
                indices.push(i);
                (sum, acc)
            })
            .1;

        let mut counts = 0;

        for (sum, indices) in &sums {
            if sum == &k {
                counts += indices.len();
            }
            let empty = Vec::new();
            let symmetric_indices = sums.get(&(sum - k)).unwrap_or(&empty);
            for i in indices {
                counts += symmetric_indices.iter().filter(|j| j < &i).count();
            }
        }

        counts as i32
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
