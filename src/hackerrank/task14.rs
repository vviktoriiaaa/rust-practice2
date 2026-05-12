/// Solution for HackerRank: Bill Division (Bon Appétit)
/// 
/// Two friends eat at a restaurant. One friend doesn't eat the k-th item.
/// This function calculates if the bill was split fairly.
pub fn calculate_bill_diff(bill: &[i32], k: usize, b: i32) -> Option<i32> {
    let total_shared: i32 = bill.iter()
        .enumerate()
        .filter(|&(i, _)| i != k)
        .map(|(_, &price)| price)
        .sum();

    let actual_share = total_shared / 2;

    if b == actual_share {
        None
    } else {
        Some(b - actual_share)
    }
}

pub fn bon_appetit(bill: &[i32], k: usize, b: i32) {
    match calculate_bill_diff(bill, k, b) {
        None => println!("Bon Appetit"),
        Some(diff) => println!("{}", diff),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fair_split() {
        let bill = vec![3, 10, 2, 9];
        let k = 1; // страву за 10 не їли
        let b = 7; // (3+2+9)/2 = 7
        assert_eq!(calculate_bill_diff(&bill, k, b), None);
    }

    #[test]
    fn test_overcharged() {
        let bill = vec![3, 10, 2, 9];
        let k = 1;
        let b = 12; // (3+10+2+9)/2 = 12, але страву k не їли
        assert_eq!(calculate_bill_diff(&bill, k, b), Some(5));
    }

    #[test]
    fn test_another_case() {
        let bill = vec![5, 5, 5];
        let k = 0;
        let b = 5; // (5+5)/2 = 5
        assert_eq!(calculate_bill_diff(&bill, k, b), None);
    }
}