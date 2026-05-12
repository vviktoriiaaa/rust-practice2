/// Solution for HackerRank: Birthday Cake Candles
/// 
/// You are in charge of the cake for a child's birthday. You have decided 
/// the cake will have one candle for each year of their total age. 
/// They will only be able to blow out the tallest of the candles. 
/// Count how many candles are tallest.
pub fn birthday_cake_candles(candles: &[i32]) -> i32 {
    if candles.is_empty() {
        return 0;
    }

    let mut max_height = 0;
    let mut count = 0;

    for &height in candles {
        if height > max_height {
            max_height = height;
            count = 1;
        } else if height == max_height {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candles_sample() {
        let candles = vec![3, 2, 1, 3];
        assert_eq!(birthday_cake_candles(&candles), 2);
    }

    #[test]
    fn test_candles_all_same() {
        let candles = vec![4, 4, 4, 4];
        assert_eq!(birthday_cake_candles(&candles), 4);
    }

    #[test]
    fn test_candles_one_tallest() {
        let candles = vec![1, 2, 5, 2, 1];
        assert_eq!(birthday_cake_candles(&candles), 1);
    }

    #[test]
    fn test_candles_empty() {
        assert_eq!(birthday_cake_candles(&[]), 0);
    }
}