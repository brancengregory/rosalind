use common::inheritance_probability;

fn main() {
    let input = common::read_input(std::env::args().skip(1));
    println!("{}", solve(&input));
}

fn solve(input: &str) -> String {
    let diploids: Vec<i32> = input.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

    let ip = inheritance_probability(diploids[0], diploids[1], diploids[2]);

    let dominant_phenotype = ip[0] + ip[1]; 

    format!("{:.prec$}", dominant_phenotype, prec = 4)
}

#[test]
fn sample() {
    let input = "2 2 2";
    assert_eq!(solve(input), "0.7833");
}
