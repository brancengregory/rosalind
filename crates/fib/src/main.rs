fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn fibonacci(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1) 
    }
}

fn fibonacci_polynomial(n: i64, k: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => (fibonacci_polynomial(n - 2, k) * k) + fibonacci_polynomial(n - 1, k) 
    }
}

fn solve(input: &str) -> String {
    let v: Vec<i64> = input.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    let n = v[0];
    let k = v[1];

    println!("n={} k={}", n, k);

    let f = fibonacci_polynomial(n, k);
    format!("{}", f)
}

#[test]
fn sample() {
    let input = "5 3";
    assert_eq!(solve(input), "19");
}

#[test]
fn basic() {
    let input = 6;
    assert_eq!(fibonacci(input), 8);
}
