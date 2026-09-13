use common::parse_fasta;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

pub fn count_transitions(s1: &str, s2: &str) -> usize {
    s1.bytes().zip(s2.bytes())
        .filter(|(b1, b2)| matches!(
            (b1, b2),
            (b'A', b'G') | (b'G', b'A') | (b'C', b'T') | (b'T', b'C')
        ))
        .count()
}

pub fn count_transversions(s1: &str, s2: &str) -> usize {
    s1.bytes().zip(s2.bytes())
        .filter(|(b1, b2)| matches!(
            (b1, b2),
            (b'A', b'C') | (b'A', b'T') | (b'C', b'A') | (b'T', b'A') |
            (b'C', b'G') | (b'G', b'C') | (b'T', b'G') | (b'G', b'T')
        ))
        .count()
}

fn solve(input: &str) -> String {
    let fasta = parse_fasta(input);
    let mut values = fasta.values();

    let s1 = values.next().unwrap();
    let s2 = values.next().unwrap();

    let transitions = count_transitions(s1, s2);
    let transversions = count_transversions(s1, s2);

    let ratio = transitions as f64 / transversions as f64;

    format!("{:.prec$}", ratio, prec = 11)
}

#[test]
fn sample() {
    let input = ">Rosalind_0209
GCAACGCACAACGAAAACCCTTAGGGACTGGATTATTTCGTGATCGTTGTAGTTATTGGA
AGTACGGGCATCAACCCAGTT
>Rosalind_2200
TTATCTGACAAAGAAAGCCGTCAACGGCTGGATAATTTCGCGATCGTGCTGGTTACTGGC
GGTACGAGTGTTCCTTTGGGT";
    assert_eq!(solve(input), "1.21428571429");
}
