/// Solution for HackerRank: Drawing Book
/// 
/// A teacher asks the class to open their books to a page number. 
/// A student can either start turning pages from the front of the book 
/// or from the back of the book. They always turn pages one at a time. 
/// When they open the book, page 1 is always on the right side.
pub fn page_count(n: i32, p: i32) -> i32 {
    // Кількість перегортань з початку
    let from_front = p / 2;
    
    // Кількість перегортань з кінця
    // Якщо n парне, остання сторінка n одна на розвороті n+1 (якого немає)
    // Якщо n непарне, n і n-1 на одному розвороті
    let from_back = (n / 2) - (p / 2);

    if from_front < from_back {
        from_front
    } else {
        from_back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count_sample1() {
        // n=6, p=2. Front: 2/2=1. Back: 6/2 - 2/2 = 3-1=2. Min: 1.
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_page_count_sample2() {
        // n=5, p=4. Front: 4/2=2. Back: 5/2 - 4/2 = 2-2=0. Min: 0.
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_page_count_even_end() {
        assert_eq!(page_count(6, 5), 1);
    }

    #[test]
    fn test_page_count_start() {
        assert_eq!(page_count(10, 1), 0);
    }
}