use std::fs;
use std::io::Read;
use std::path::Path;

/// Read input from an optional file path arg, falling back to stdin.
/// Trims surrounding whitespace (Rosalind datasets end with a newline).
pub fn read_input(args: impl Iterator<Item = String>) -> String {
    let raw = match args.into_iter().next() {
        Some(path) => fs::read_to_string(Path::new(&path)).expect("failed to read input file"),
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .expect("failed to read stdin");
            buf
        }
    };
    raw.trim().to_string()
}

pub fn count_nucleotides(dna: &str) -> [i32; 4] {
    let mut nucleotides: [i32; 4] = [0, 0, 0, 0];

    dna.chars().for_each(|c| {
        match c {
            'A' => nucleotides[0] += 1,
            'C' => nucleotides[1] += 1,
            'G' => nucleotides[2] += 1,
            'T' => nucleotides[3] += 1,
            _ => panic!("unexpected character")
        }
    });

    nucleotides
}

pub fn transcribe(dna: &str) -> String {
    dna.chars()
        .map(|c| {
            if c == 'T' {'U'} else {c}
        })
        .collect()
}

pub fn complement(dna: &str) -> String {
    dna.chars()
        .rev()
        .map(|c| {
            match c {
                'A' => 'T',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => panic!("unexpected character")
            }
        })
        .collect()
}

pub fn allele_count(dominant: i32, hetero: i32, recessive: i32) -> [i32; 2] {
    [2 * dominant + hetero, 2 * recessive + hetero]
}

pub fn allele_frequency(population: i32, diploids: [i32; 3]) -> [f32; 2] {
    let allele_count = allele_count(diploids[0], diploids[1], diploids[2]);

    [allele_count[0] as f32 / population as f32, allele_count[1] as f32 / population as f32]
}

pub fn inheritance_probability(dominant: i32, hetero: i32, recessive: i32) -> [f64; 3] {
    let k = dominant as f64;
    let m = hetero as f64;
    let n = recessive as f64;
    let t = k + m + n;

    let dom_dom = (k / t) * ((k - 1.0) / (t - 1.0));
    let rec_rec = (n / t) * ((n - 1.0) / (t - 1.0));
    let het_het = (m / t) * ((m - 1.0) / (t - 1.0));
    let dom_het = 2.0 * (k / t) * (m / (t - 1.0));
    let dom_rec = 2.0 * (k / t) * (n / (t - 1.0));
    let het_rec = 2.0 * (m / t) * (n / (t - 1.0));

    let aa_homo_dom = (dom_dom * 1.0) + (dom_het * 0.5) + (het_het * 0.25);
    let aa_hetero = (dom_het * 0.5) + (dom_rec * 1.0) + (het_het * 0.5) + (het_rec * 0.5);
    let aa_homo_rec = (het_het * 0.25) + (het_rec * 0.5) + (rec_rec * 1.0);

    [aa_homo_dom, aa_hetero, aa_homo_rec]
}

#[cfg(test)]
mod tests {
//    use super::*;
}
