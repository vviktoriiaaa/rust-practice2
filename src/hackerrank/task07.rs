use std::io;

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let temp = y;
            y = x % y;
            x = temp;
        }
        x
    }

    fn lcm(x: i32, y: i32) -> i32 {
        x * y / gcd(x, y)
    }

    let mut lcm_a = a[0];
    for &num in a.iter() {
        lcm_a = lcm(lcm_a, num);
    }

    let mut gcd_b = b[0];
    for &num in b.iter() {
        gcd_b = gcd(gcd_b, num);
    }

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let mut input = String::new();

    // читаємо перший рядок (розміри масивів)
    io::stdin().read_line(&mut input).unwrap();
    let sizes: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();

    // читаємо масив A
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();

    // читаємо масив B
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = getTotalX(&a, &b);
    println!("{}", result);
}