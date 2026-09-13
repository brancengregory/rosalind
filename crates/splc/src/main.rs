use common::{parse_fasta, translate_rna, dna_to_rna};

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let fasta = parse_fasta(input);

    let dna = fasta.values().next().unwrap().as_str();
    let introns: Vec<&str> = fasta.values().skip(1).map(|s| s.as_str()).collect();

    let mut rest = dna.to_string();

    for i in introns {
        rest = rest.replace(i, "");
    }

    let rna = dna_to_rna(&rest);
    translate_rna(&rna)
}

#[test]
fn sample() {
    let input = ">Rosalind_10
ATGGTCTACATAGCTGACAAACAGCACGTAGCAATCGGTCGAATCTCGAGAGGCATATGGTCACATGATCGGTCGAGCGTGTTTCAAAGTTTGCGCCTAG
>Rosalind_12
ATCGGTCGAA
>Rosalind_15
ATCGGTCGAGCGTGT";
    assert_eq!(solve(input), "MVYIADKQHVASREAYGHMFKVCA");
}
