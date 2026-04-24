// https://www.hackerrank.com/challenges/migratory-birds/problem
/// Solution for HackerRank: Migratory Birds
/// 
/// Identifies the most frequently sighted bird type. 
/// If multiple types have the same maximum count, the smallest ID is returned.
pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut frequency_map = vec![0; 6];
    
    // Заповнюємо частотний масив
    for &id in arr {
        if id > 0 && id < 6 {
            frequency_map[id as usize] += 1;
        }
    }

    let mut most_frequent_id = 1;
    let mut max_sightings = 0;

    // Шукаємо найменший ID з максимальним результатом
    for id in 1..6 {
        if frequency_map[id] > max_sightings {
            max_sightings = frequency_map[id];
            most_frequent_id = id as i32;
        }
    }

    most_frequent_id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birds_general_case() {
        let sightings = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&sightings), 4);
    }

    #[test]
    fn test_birds_tie_break() {
        let sightings = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&sightings), 3);
    }

    #[test]
    fn test_birds_all_unique() {
        let sightings = vec![5, 4, 3, 2, 1];
        assert_eq!(migratory_birds(&sightings), 1);
    }

    #[test]
    fn test_birds_uniform_sightings() {
        let sightings = vec![2, 2, 2, 2];
        assert_eq!(migratory_birds(&sightings), 2);
    }
}