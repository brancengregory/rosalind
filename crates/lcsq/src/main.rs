use common::parse_fasta;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

pub fn longest_common_subsequence(strings: Vec<&str>) -> String {
    let mut strings_c = strings.clone();
    strings_c.sort_by_key(|s| s.len());

    let shortest_string = &strings_c[0];
    let other_strings = &strings_c[1..];

    let max_size = shortest_string.len();

    for len in (1..=max_size).rev() {
        for start in 0..=(max_size - len) {
            let subseq = &shortest_string[start..start + len];

            if other_strings.iter().all(|s| s.contains(subseq)) {
                return subseq.to_string()
            } 
        }
    }

    String::new()
}

fn solve(input: &str) -> String {
    let fasta = parse_fasta(input);
    let strings: Vec<&str> = fasta.values().map(|s| s.as_str()).collect();

    longest_common_subsequence(strings)
}

#[test]
fn sample() {
    let input = ">Rosalind_23
AACCTTGG
>Rosalind_64
ACACTGTGA";
    assert_eq!(solve(input), "AACTGG");
}
