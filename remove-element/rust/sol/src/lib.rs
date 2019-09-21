struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = nums.len() as i32;
        let mut i = 0;

        while (i as i32) < count {
            if nums[i] == val {
                nums.swap_remove(i);
                count -= 1;
            } else {
                i += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(2, Solution::remove_element(&mut nums, val));
        assert_eq!(vec![2, 2], nums);
    }

    #[test]
    fn ex2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        assert_eq!(5, Solution::remove_element(&mut nums, val));
        assert_eq!(vec![0, 1, 4, 0, 3], nums);
    }

    #[test]
    fn empty() {
        let mut nums = vec![];
        let val = 2;
        assert_eq!(0, Solution::remove_element(&mut nums, val));
        assert_eq!(vec![] as Vec<i32>, nums);
    }

    #[test]
    fn no_match() {
        let mut nums = vec![0, 1, 2, 3];
        let val = 4;
        assert_eq!(4, Solution::remove_element(&mut nums, val));
        assert_eq!(vec![0, 1, 2, 3], nums);
    }

    #[test]
    fn only_matches() {
        let mut nums = vec![0, 0, 0, 0];
        let val = 0;
        assert_eq!(0, Solution::remove_element(&mut nums, val));
        assert_eq!(vec![] as Vec<i32>, nums);
    }
}
