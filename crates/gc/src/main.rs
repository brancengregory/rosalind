use common::{parse_fasta, gc_content};

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let fasta = parse_fasta(input);

    let max_gc = fasta.iter()
        .map(|(k, v)| (k, gc_content(v)))
        .max_by(|(_, gc1), (_, gc2)| {
            gc1.total_cmp(gc2)
        });

    let (id, gc) = max_gc.unwrap();
    format!("{}\n{:.prec$}", id, gc, prec = 6)
}

#[test]
fn sample() {
    let input = "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT";
    assert_eq!(format!("{:.prec$}", gc_content(input), prec = 6), "60.919540");
}

#[test]
fn full() {
    let input = r"
        >Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT
    ";
    assert_eq!(solve(input), "Rosalind_0808\n60.919540");
}
