use common::hamming_distance;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let s1 = lines.next().unwrap();
    let s2 = lines.next().unwrap();

    let dist = hamming_distance(s1, s2);

    format!("{}", dist)
}

#[test]
fn sample() {
    let input = "GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT";
    assert_eq!(solve(input), "7");
}
