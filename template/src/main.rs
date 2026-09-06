fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let _ = input;
    todo!("solve {ID}")
}

#[test]
fn sample() {
    let input = "";
    assert_eq!(solve(input), "");
}
