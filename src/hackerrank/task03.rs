/// ПРАКТИЧНА 4: Grading Students
/// Автор: vviktoriiaaa
/// Логіка: Округлення оцінок до найближчого числа, кратного 5

pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut processed_grades = Vec::with_capacity(grades.len());

    for &score in grades {
        // Якщо бал менше 38, він не підлягає округленню
        if score < 38 {
            processed_grades.push(score);
            continue;
        }

        let next_multiple_of_5 = ((score / 5) + 1) * 5;
        
        // Округляємо, якщо різниця менша за 3
        if next_multiple_of_5 - score < 3 {
            processed_grades.push(next_multiple_of_5);
        } else {
            processed_grades.push(score);
        }
    }

    processed_grades
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_university_grading() {
        let input = [73, 67, 38, 33];
        let output = grading_students(&input);
        assert_eq!(output, vec![75, 67, 40, 33]);
    }

    #[test]
    fn test_rounding_threshold() {
        assert_eq!(grading_students(&[37]), vec![37]); // < 38 не міняємо
        assert_eq!(grading_students(&[82]), vec![82]); // 85-82=3 (не менше 3)
    }
}