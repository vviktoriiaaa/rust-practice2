use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/// ПРАКТИЧНА 6: Number Line Jumps

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Якщо швидкості однакові, вони зустрінуться тільки якщо стартанули з однієї точки
    if v1 == v2 {
        return if x1 == x2 { "YES".to_string() } else { "NO".to_string() };
    }

    // Розраховуємо різницю позицій та швидкостей
    let pos_diff = x1 - x2;
    let speed_diff = v2 - v1;

    // Перевіряємо, чи рухається той, хто позаду, швидше 
    // і чи наздожене він другого рівно в цілій точці
    if (pos_diff.signum() == speed_diff.signum()) && (pos_diff % speed_diff == 0) {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let output_path = env::var("OUTPUT_PATH").unwrap();
    let mut fptr = File::create(output_path).unwrap();

    let input_line = stdin_iterator.next().unwrap().unwrap();
    let data: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = kangaroo(data[0], data[1], data[2], data[3]);

    writeln!(&mut fptr, "{}", result).ok();
}