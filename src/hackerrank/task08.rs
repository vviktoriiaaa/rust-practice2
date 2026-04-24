// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
/// Solution for HackerRank: Breaking the Records
/// 
/// Calculates how many times high and low records were broken.
pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut highest = scores[0];
    let mut lowest = scores[0];
    let (mut high_breaks, mut low_breaks) = (0, 0);

    // Використовуємо ітератор зі skip, щоб пропустити першу гру
    scores.iter().skip(1).for_each(|&current_score| {
        if current_score > highest {
            highest = current_score;
            high_breaks += 1;
        } else if current_score < lowest {
            lowest = current_score;
            low_breaks += 1;
        }
    });

    vec![high_breaks, low_breaks]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_breaks_standard() {
        let data = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&data), vec![2, 4]);
    }

    #[test]
    fn test_record_breaks_long_season() {
        let data = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&data), vec![4, 0]);
    }

    #[test]
    fn test_no_scores() {
        assert_eq!(breaking_records(&[]), vec![0, 0]);
    }

    #[test]
    fn test_constant_scores() {
        assert_eq!(breaking_records(&[10, 10, 10]), vec![0, 0]);
    }
}