use std::fs;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

use aho_corasick::AhoCorasick;
use indexmap::IndexMap;

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

pub fn gc_content(dna: &str) -> f64 {
    let (total, gc) = dna.chars().fold((0, 0), |(total, gc), x| {
        match x {
            'G' | 'C' => (total + 1, gc + 1),
            'T' | 'A' => (total + 1, gc),
            _ => panic!()
        }
    });

    100_f64 * (gc as f64 / total as f64)
}

pub fn parse_fasta(s: &str) -> IndexMap::<String, String> {
    let mut im: IndexMap<String, String> = IndexMap::new();
    let mut current_id = String::new();

    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue
        }

        if let Some(id) = line.strip_prefix('>') {
            current_id = id.to_string();
            im.entry(current_id.clone()).or_default();
        } else if let Some(seq) = im.get_mut(&current_id) {
            seq.push_str(line)
        }
    }

    im
}

pub fn translate_codon(codon: &str) -> Option<char> {
    match codon {
        // Phenylalanine (F)
        "UUU" | "UUC" => Some('F'),

        // Leucine (L)
        "UUA" | "UUG" | "CUU" | "CUC" | "CUA" | "CUG" => Some('L'),

        // Isoleucine (I)
        "AUU" | "AUC" | "AUA" => Some('I'),

        // Methionine / Start (M)
        "AUG" => Some('M'),

        // Valine (V)
        "GUU" | "GUC" | "GUA" | "GUG" => Some('V'),

        // Serine (S)
        "UCU" | "UCC" | "UCA" | "UCG" | "AGU" | "AGC" => Some('S'),

        // Proline (P)
        "CCU" | "CCC" | "CCA" | "CCG" => Some('P'),

        // Threonine (T)
        "ACU" | "ACC" | "ACA" | "ACG" => Some('T'),

        // Alanine (A)
        "GCU" | "GCC" | "GCA" | "GCG" => Some('A'),

        // Tyrosine (Y)
        "UAU" | "UAC" => Some('Y'),

        // Histidine (H)
        "CAU" | "CAC" => Some('H'),

        // Glutamine (Q)
        "CAA" | "CAG" => Some('Q'),

        // Asparagine (N)
        "AAU" | "AAC" => Some('N'),

        // Lysine (K)
        "AAA" | "AAG" => Some('K'),

        // Aspartic Acid (D)
        "GAU" | "GAC" => Some('D'),

        // Glutamic Acid (E)
        "GAA" | "GAG" => Some('E'),

        // Cysteine (C)
        "UGU" | "UGC" => Some('C'),

        // Tryptophan (W)
        "UGG" => Some('W'),

        // Arginine (R)
        "CGU" | "CGC" | "CGA" | "CGG" | "AGA" | "AGG" => Some('R'),

        // Glycine (G)
        "GGU" | "GGC" | "GGA" | "GGG" => Some('G'),

        // Stop Codons
        "UAA" | "UAG" | "UGA" => None,

        // Invalid or unknown codon
        _ => None,
    }
}

pub fn translate_rna(rna: &str) -> String {
    let mut protein_str = String::new();

    for codon in rna.as_bytes().chunks_exact(3) {
        if let Some(s) = translate_codon(str::from_utf8(codon).unwrap()) {
            protein_str.push(s);
        }
    }

    protein_str
}

pub fn substring_positions(s: &str, qs: Vec<&str>) -> Vec<usize> {
    let ac = AhoCorasick::builder()
        .match_kind(aho_corasick::MatchKind::Standard)
        .build(qs)
        .unwrap();

    ac.find_overlapping_iter(s).map(|m| m.start()).collect()
}

pub fn hamming_distance(s1: &str, s2: &str) -> usize {
    if !s1.len().eq(&s2.len()) {
        panic!()
    }

    s1.chars().zip(s2.chars()).fold(0, |acc, (c1, c2)| {
        if c1 != c2 {acc + 1} else {acc}
    })
}

pub fn dna_to_rna(dna: &str) -> String {
    dna.replace('T', "U")
}

#[cfg(test)]
mod tests {
//    use super::*;
}
