struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows: HashSet<usize> = HashSet::new();
        let mut cols: HashSet<usize> = HashSet::new();

        for (i, row) in matrix.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if elem == &0 {
                    rows.insert(i);
                    cols.insert(j);
                }
            }
        }

        for row in rows.iter() {
            for col in cols.iter() {
                Solution::clear_row(matrix, *row);
                Solution::clear_col(matrix, *col);
            }
        }
    }

    fn clear_col(matrix: &mut Vec<Vec<i32>>, j: usize) {
        for row in matrix.iter_mut() {
            row[j] = 0;
        }
    }

    fn clear_row(matrix: &mut Vec<Vec<i32>>, i: usize) {
        for elem in matrix[i].iter_mut() {
            *elem = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ex1() {
        let mut matrix = vec![
            vec![1,1,1],
            vec![1,0,1],
            vec![1,1,1]
        ];
        let expected = vec![
            vec![1,0,1],
            vec![0,0,0],
            vec![1,0,1]
        ];

        Solution::set_zeroes(&mut matrix);
        assert_eq!(expected, matrix);
    }

    #[test]
    fn ex2() {
        let mut matrix = vec![
            vec![0,1,2,0],
            vec![3,4,5,2],
            vec![1,3,1,5]
        ];
        let expected = vec![
            vec![0,0,0,0],
            vec![0,4,5,0],
            vec![0,3,1,0]
        ];

        Solution::set_zeroes(&mut matrix);
        assert_eq!(expected, matrix);
    }

    #[test]
    fn empty() {
        let mut matrix = vec![];
        let expected: Vec<Vec<i32>> = vec![];

        Solution::set_zeroes(&mut matrix);
        assert_eq!(expected, matrix);
    }
}
