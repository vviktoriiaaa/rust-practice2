// https://www.hackerrank.com/challenges/sock-merchant/problem
/// Solution for HackerRank: Sales by Match
/// 
/// Finds the number of pairs of socks with matching colors.
/// Uses sorting approach to identify adjacent matches.
pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    if ar.is_empty() {
        return 0;
    }

    let mut sorted_socks = ar.to_vec();
    sorted_socks.sort_unstable();

    let mut pair_count = 0;
    let mut i = 0;

    while i < sorted_socks.len() - 1 {
        if sorted_socks[i] == sorted_socks[i + 1] {
            pair_count += 1;
            i += 2; // Пропускаємо обидві шкарпетки з пари
        } else {
            i += 1; // Переходимо до наступної
        }
    }

    pair_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socks_basic_pairing() {
        let n = 9;
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(n, &ar), 3);
    }

    #[test]
    fn test_socks_mixed_set() {
        let n = 10;
        let ar = vec![1, 1, 3, 1, 2, 1, 3, 3, 3, 3];
        assert_eq!(sock_merchant(n, &ar), 4);
    }

    #[test]
    fn test_no_matches() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(5, &ar), 0);
    }

    #[test]
    fn test_empty_pile() {
        assert_eq!(sock_merchant(0, &[]), 0);
    }
}