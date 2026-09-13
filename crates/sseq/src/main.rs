use common::parse_fasta;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let fasta = parse_fasta(input);

    let mut vals = fasta.values();
    let s = vals.next().unwrap();
    let s_bytes = s.as_bytes();
    let q = vals.next().unwrap(); 

    let mut indices: Vec<usize> = Vec::new();
    let mut s_idx = 0;

    for c in q.as_bytes() {
        while s_idx < s_bytes.len() && &s_bytes[s_idx] != c {
            s_idx += 1;
        }
        
        indices.push(s_idx + 1);
        s_idx += 1;
    } 

    let mut f = String::new();
    for i in indices {
        f.push_str(&format!("{} ", i))
    }

    f.trim().to_string()
}

#[test]
fn sample() {
    let input = ">Rosalind_14
ACGTACGTGACG
>Rosalind_18
GTA";
    assert_eq!(solve(input), "3 4 5");
}
