use common::translate_rna;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    translate_rna(input)
}

#[test]
fn sample() {
    let input = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
    assert_eq!(solve(input), "MAMAPRTEINSTRING");
}
