/// ПРАКТИЧНА 3: Staircase
/// Виконано для: vviktoriiaaa
/// Опис: Побудова "сходів" із символів # з вирівнюванням по правому краю

pub fn staircase(n: i32) -> String {
    let mut output = Vec::new();

    for i in 1..=n {
        let spaces_count = (n - i) as usize;
        let hashes_count = i as usize;
        
        // Створюємо рядок: спочатку пробіли, потім решітки
        let line = format!("{}{}", " ".repeat(spaces_count), "#".repeat(hashes_count));
        output.push(line);
    }

    let final_string = output.join("\n") + "\n";
    print!("{}", final_string); // Вивід у консоль за вимогою задачі
    final_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_staircase_viki() {
        let result = staircase(4);
        let expected = "   #\n  ##\n ###\n####\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_step() {
        assert_eq!(staircase(1), "#\n");
    }
}