use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

fn solve(input: Vec<(&str, u32)>, solution: &[u32]) -> String {
    // Build symbol table
    let mut symbols = BTreeSet::new();
    input.iter().for_each(|(i, _)| {
        i.to_uppercase().chars().for_each(|c| {
            symbols.insert(c);
        })
    });

    let symbol_index = symbols
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect::<BTreeMap<char, usize>>();

    dbg!(&symbols.len());

    // B
    let b = input.iter().map(|(_, b)| *b).collect::<Vec<_>>();

    // Setup A (input.len() x symbols.len())
    let mut a = vec![];
    for (i, _) in input {
        let mut a_in = (0..symbols.len()).map(|_| 0).collect::<Vec<_>>();
        for c in i.to_uppercase().chars() {
            a_in[symbol_index[&c]] += 1;
        }
        a.push(a_in);
    }

    let sym_vec: Vec<char> = Vec::from_iter(symbols);
    for p in (0..sym_vec.len()).permutations(sym_vec.len()) {
        if check_solution(&a, &p, &b) {
            return solution
                .iter()
                .map(|&sym| p.iter().position(|&x| x as u32 == sym).unwrap())
                .map(|x| sym_vec[x])
                .collect::<String>()
                .to_uppercase();
        }
    }

    "".to_owned()
}

fn check_solution(a: &Vec<Vec<u32>>, x: &Vec<usize>, b: &Vec<u32>) -> bool {
    for row in 0..a.len() {
        if a[row]
            .iter()
            .zip(x)
            .map(|(a, x)| a * (*x as u32))
            .sum::<u32>()
            != b[row]
        {
            return false;
        }
    }
    true
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn test1() {
        let input = vec![
            ("BOOGIE", 58),
            ("BOSSANOVA", 66),
            ("CHACHACHA", 18),
            ("CHARLESTON", 76),
            ("FLAMENCO", 62),
            ("FOXTROTT", 102),
            ("JIVE", 69),
            ("LAMBADA", 29),
            ("MAMBO", 20),
            ("PASODOBLE", 63),
            ("QUICKSTEP", 118),
            ("ROCKNROLL", 88),
            ("RUMBA", 43),
            ("SAMBA", 18),
            ("SHIMMY", 59),
            ("SIRTAKI", 83),
            ("TANGO", 47),
            ("TWIST", 68),
            ("WALZER", 80),
        ];

        let solution = [9, 16, 7, 2, 6, 8, 16, 6, 22];

        let s = solve(input, &solution);
        assert_eq!(s, "FOXTROTT");
    }
}
