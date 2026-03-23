/// ПРАКТИЧНА 4: Grading Students
/// https://www.hackerrank.com/challenges/grading

pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| {
        if grade >= 38 {
            let rem = grade % 5;
            if rem >= 3 {
                return grade + (5 - rem);
            }
        }
        grade
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grading() {
        assert_eq!(grading_students(&[73, 67, 38, 33]), vec![75, 67, 40, 33]);
    }
}