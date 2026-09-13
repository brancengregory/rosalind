use common::substring_positions;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let s = lines.next().unwrap();
    let q = lines.next().unwrap();

    let matches: Vec<usize> = substring_positions(s, vec!(q));

    let mut f = String::new(); 
    for m in matches {
        f.push_str(&format!("{} ", m + 1))
    }

    f.trim_end().to_string()
}

#[test]
fn sample() {
    let input = "GATATATGCATATACTT
ATAT";
    assert_eq!(solve(input), "2 4 10");
}
