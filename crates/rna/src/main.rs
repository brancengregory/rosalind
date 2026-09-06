use common::transcribe;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    transcribe(input)
}

#[test]
fn sample() {
    let input = "GATGGAACTTGACTACGTAAATT";
    assert_eq!(solve(input), "GAUGGAACUUGACUACGUAAAUU");
}
