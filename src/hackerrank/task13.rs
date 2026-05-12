/// Solution for HackerRank: Divisible Sum Pairs
/// 
/// Given an array of integers and a positive integer k, determine the number of (i, j) pairs 
/// where i < j and ar[i] + ar[j] is divisible by k.
pub fn divisible_sum_pairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    let n = n as usize;

    for i in 0..n {
        for j in (i + 1)..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_sum_pairs_standard() {
        let n = 6;
        let k = 3;
        let ar = vec![1, 3, 2, 6, 1, 2];
        // Pairs: (1,2), (1,5), (3,6), (6,12) -> [1,2], [1,2], [3,6], [2,1] etc.
        // In this case: (1,2), (1,2), (3,6), (2,1), (1,2) ... 
        // Actual pairs indices for [1, 3, 2, 6, 1, 2] with k=3:
        // (0,2) -> 1+2=3; (0,5) -> 1+2=3; (1,3) -> 3+6=9; (2,4) -> 2+1=3; (4,5) -> 1+2=3
        assert_eq!(divisible_sum_pairs(n, k, &ar), 5);
    }

    #[test]
    fn test_no_pairs() {
        let ar = vec![1, 1, 1];
        assert_eq!(divisible_sum_pairs(3, 5, &ar), 0);
    }

    #[test]
    fn test_small_array() {
        let ar = vec![8, 10];
        assert_eq!(divisible_sum_pairs(2, 2, &ar), 1);
    }
}