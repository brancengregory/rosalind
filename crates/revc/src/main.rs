use common::complement;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    complement(input)
}

#[test]
fn sample() {
    let input = "AAAACCCGGT";
    assert_eq!(solve(input), "ACCGGGTTTT");
}
