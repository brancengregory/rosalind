# Rosalind

Rust solutions for the [Rosalind](https://rosalind.info/problems/list-view/) bioinformatics challenges.

## Layout

- `crates/<id>/` — one binary crate per problem, named by Rosalind ID (lowercase, e.g. `dna`).
- `crates/common/` — shared utilities: stdin/file input, FASTA parser, nucleotide helpers.
- `data/<id>/` — downloaded datasets (`input.txt`); sample tests are inline `#[test]`s in each crate.
- `template/` — crate skeleton used by the scaffolding script.
- `bin/new-problem.sh` — scaffold a new problem crate.

## Usage

Each binary reads from a file path argument, falling back to stdin:

```sh
cargo run -p <id> -- data/<id>/input.txt
cargo run -p <id> < data/<id>/input.txt
```

## Fetching datasets

```sh
bin/fetch.sh dna            # writes data/dna/input.txt
bin/fetch.sh dna -c cookies.txt
```

Datasets live at `rosalind.info/media/rosalind_<id>.txt` and require a logged-in
session. The script looks for a cookie file, in order:

1. `-c <file>` on the command line (Netscape format)
2. `$ROSALIND_COOKIE` env var (path to a cookie file)
3. `~/.rosalind-cookie.txt`
4. `$ROSALIND_SESSION` env var (just the `sessionid` value)

To export a cookie from a logged-in browser, copy the `sessionid` value from
devtools and run:

```sh
ROSALIND_SESSION=<sessionid> bin/fetch.sh dna
```

## Adding a problem

```sh
bin/new-problem.sh revc    # creates crates/revc/ and data/revc/
```

Then: download the dataset into `data/revc/input.txt`, implement `crates/revc/src/main.rs`, and test the sample dataset with inline `#[test]`s.

## Progress

| ID | Problem | Status |
|----|---------|--------|
| —  | none yet | — |
