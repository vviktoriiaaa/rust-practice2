/// Solution for HackerRank: Diagonal Difference
/// 
/// Calculates the absolute difference between the sums of the matrix diagonals.
/// Uses enumeration for clean indexing.
pub fn diagonal_difference(matrix: &[Vec<i32>]) -> i32 {
    let size = matrix.len();
    let mut left_to_right = 0;
    let mut right_to_left = 0;

    for (i, row) in matrix.iter().enumerate() {
        left_to_right += row[i];
        right_to_left += row[size - 1 - i];
    }

    (left_to_right - right_to_left).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_standard_matrix() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![9, 8, 9],
        ];
        // (1+5+9) - (3+5+9) = 15 - 17 = |-2| = 2
        assert_eq!(diagonal_difference(&matrix), 2);
    }

    #[test]
    fn test_diagonal_negative_numbers() {
        let matrix = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];
        assert_eq!(diagonal_difference(&matrix), 15);
    }

    #[test]
    fn test_diagonal_minimal_matrix() {
        let matrix = vec![
            vec![1, 0],
            vec![0, 1],
        ];
        assert_eq!(diagonal_difference(&matrix), 2);
    }
}