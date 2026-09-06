use common::count_nucleotides;


fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let output = count_nucleotides(input);
    format!("{} {} {} {}", output[0], output[1], output[2], output[3])
}

#[test]
fn sample() {
    let input = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
    assert_eq!(solve(input), "20 12 17 21");
}
