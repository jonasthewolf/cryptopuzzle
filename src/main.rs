use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
    num::ParseIntError,
};

use itertools::Itertools;

///
/// Solve the crypto puzzle
///
///
fn solve(input: &[(&str, u32)], solution: &[u32]) -> String {
    // Build symbol table
    let mut symbols = BTreeSet::new();
    input.iter().for_each(|(i, _)| {
        i.to_uppercase().chars().for_each(|c| {
            symbols.insert(c);
        })
    });

    let sym_len = symbols.len();
    let mut candidates = BTreeMap::new();
    for &s in &symbols {
        candidates.insert(s, BTreeSet::from_iter(1..=sym_len));
    }

    dbg!(sym_len);

    // Extract sum of each row
    let row_sum = input.iter().map(|(_, b)| *b).collect::<Vec<_>>();

    // Translate input strings
    let mut translated_symbols = vec![];
    for (i, _) in input {
        let mut a_in = BTreeMap::new();
        for c in i.to_uppercase().chars() {
            let e: &mut u32 = a_in.entry(c).or_default();
            *e += 1;
        }
        translated_symbols.push(a_in);
    }

    // FIXME: 20 is nonsense. Do until all candidates have only single value left.
    // while &candidates.values().all(|x| x.len() == 1) {
    // Optimization would be if all numbers from "solution" array are resolved.
    for u in 0..12 {
        dbg!(u);
        print_candidates(&candidates);
        
        reduce_by_max_value(&mut candidates, &row_sum, &translated_symbols);
        
        println!("After reduce_by_max_value");
        print_candidates(&candidates);

        remove_hidden_tuples(&mut candidates, input);

        remove_singles(&mut candidates);
    }

    // Convert list of numbers from solution to characters
    solution
        .iter()
        .map(|&x| {
            candidates
                .iter()
                .find_map(|(c, cand)| {
                    if cand.contains(&(x as usize)) {
                        // TOOD maybe better first()
                        Some(*c)
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .collect::<String>()
        .to_uppercase()
}

fn remove_singles(candidates: &mut BTreeMap<char, BTreeSet<usize>>) {
    let mut toberemoved = vec![];
    candidates.iter().for_each(|(ccand, cands)| {
        if cands.len() == 1 {
            toberemoved.push((*ccand, *cands.first().unwrap()));
        }
    });
    for (tbrc, tbrv) in toberemoved {
        candidates.iter_mut().for_each(|(c, v)| {
            if *c != tbrc {
                v.remove(&tbrv);
            }
        });
    }
}

///
/// Reduce candidates by looking into each row and calculate if the lowest values possible are more than b
///
///
fn reduce_by_max_value(
    candidates: &mut BTreeMap<char, BTreeSet<usize>>,
    row_sum: &[u32],
    translated_symbols: &[BTreeMap<char, u32>],
) {
    // Reduce candidates
    // Multiply minimum of others candidates and add them up.
    // This sum - b is the maximum of candidate values for current symbol.
    // Go through all input strings
    for (i, frequencies) in translated_symbols.iter().enumerate() {
        let mut orderedfrequencies = Vec::from_iter(frequencies);
        orderedfrequencies.sort_by(|x, y| x.1.cmp(y.1).reverse());
        // Loop over all characters in input string starting with the most frequent
        // and see what the maximum value is it can take
        // if the others take their current minimum value
        // taking into account that they all have to have different values
        for (cs, _) in &orderedfrequencies {
            let mut selectedcands: BTreeMap<char, usize> = BTreeMap::new();
            let mut max = 0;

            // Add the other minima taking into account the already chosen values
            // Since they are ordered by their frequency (which is in turn the factor for their value)
            for (x, &v) in &orderedfrequencies {
                if x != cs {
                    if let Some(curcand) = candidates[x]
                        .iter()
                        .filter(|f| !selectedcands.values().contains(f))
                        .min()
                    {
                        max += *curcand as u32 * v;
                        selectedcands.insert(**x, *curcand);
                    }
                }
            }

            candidates
                .get_mut(cs)
                .unwrap() // unwrap ok, since key definitely exists
                .retain(|&v| {
                    (frequencies[cs] * (v as u32) + max <= row_sum[i])
                        || selectedcands.values().contains(&v)
                });
        }
    }
}

///
/// Print current candidates list
///
fn print_candidates(candidates: &BTreeMap<char, BTreeSet<usize>>) {
    for (c, cs) in candidates.iter().sorted_by(|a, b| Ord::cmp(a.0, b.0)) {
        println!("{c} : {}", cs.iter().join(","));
    }
}

///
/// Get real subsets in candidates with length tuple_len
///
/// TODO If two subsets are completely disjoint, the algorithm fails
/// Actually, we are not interested if they are disjoint, however, the subsequent check
/// if number of tuples is equal to tuple_len fails if there are disjoint sets.
/// TODO Get the sets and see then how many of them are disjoint
/// Example A: 2,3; B: 2,3; C: 4,5; D: 4,5; tuple_len = 2
///
///
fn get_real_subsets(
    candidates: &BTreeMap<char, BTreeSet<usize>>,
    tuple_len: usize,
) -> Vec<BTreeSet<char>> {
    let tuples = candidates
        .iter()
        .filter(|(_, p)| p.len() <= tuple_len) // && p.len() > 1) // p.len() > 1 is necessary to prevent from singletons being removed
        .collect::<Vec<_>>();
    let mut result: Vec<BTreeSet<char>> = Vec::new();

    for (&first_char, first) in &tuples {
        // dbg!(&first_char);
        // dbg!(&first);
        for (&second_char, second) in &tuples {
            if second_char != first_char {
                // dbg!(&set);
                // dbg!(c);
                if (second.is_superset(first) && first.is_subset(second))
                    || (first.is_superset(second) && second.is_subset(first))
                {
                    let mut new_set = true;
                    for set in result.iter_mut() {
                        if set.contains(&first_char) || set.contains(&second_char) {
                            // dbg!(first_char);
                            // dbg!(second_char);
                            // dbg!(&set);
                            set.insert(first_char);
                            set.insert(second_char);
                            new_set = false;
                            break;
                        }
                    }
                    if new_set {
                        result.push(BTreeSet::from([first_char, second_char]));
                    }
                }
            }
        }
        // dbg!(&result);
    }
    result
}

///
///  Find hidden tuples
///
///  tuple length 1 is the special case where a symbol has been identified.
///
fn remove_hidden_tuples(candidates: &mut BTreeMap<char, BTreeSet<usize>>, input: &[(&str, u32)]) {
    // Start from highest value to remove largest tuple first
    for tuple_len in (2..input.len()-1).rev() { // -1 because it does not make sense to start with all tuples
        let subsets = get_real_subsets(candidates, tuple_len);
        for subset in subsets {
            // Number of symbols has to match the number of tuples to be a hidden tuple
            if subset.len() == tuple_len {
                let sub = subset
                    .iter()
                    .flat_map(|x| candidates[x].iter().map(|z| *z))
                    .collect::<BTreeSet<_>>();
                for (c, cands) in candidates.iter_mut() {
                    if !subset.contains(c) {
                        cands.retain(|v| !sub.contains(v));
                    }
                }
            }
        }
    }
}

///
/// Entry point into program
///
///
fn main() -> Result<(), ParseIntError> {
    let mut lines = std::io::stdin().lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let solution: Vec<u32> = line
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut input = vec![];
        while let Some(Ok(line)) = lines.next() {
            let inp = line.split(&[',', ' ']).collect::<Vec<_>>();
            input.push((inp[0].to_owned(), inp[1].parse::<u32>()?));
        }
        let input = input
            .iter()
            .map(|(a, b)| (a.as_str(), *b))
            .collect::<Vec<_>>();
        println!("{}", solve(&input, &solution));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, BTreeSet};

    use crate::{get_real_subsets, solve};

    #[test]
    fn puzzle1() {
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

        let solution = [9, 16, 7, 2, 6, 18, 6, 22];

        let s = solve(&input, &solution);
        assert_eq!(s, "DISCOFOX");

        // A=1, B=5, C=2, D=9, E=10, F=18, G=15, H=3, I=16, J=23, K=17, L=8, M=4
        // N=13, O=6, P=11, Q=24, R=14, S=7, T=12, U=19, V=20, W=21, X=22, Y=25, Z=26
    }

    #[test]
    fn realsubset() {
        let mut candidates = BTreeMap::new();
        let tuple_len = 3;

        candidates.insert('e', BTreeSet::from([2, 6, 7, 8, 9]));
        candidates.insert('a', BTreeSet::from([4, 5, 6]));
        candidates.insert('g', BTreeSet::from([9]));
        candidates.insert('b', BTreeSet::from([4, 6]));
        candidates.insert('c', BTreeSet::from([4, 5, 6]));
        candidates.insert('d', BTreeSet::from([1, 2, 3]));
        candidates.insert('f', BTreeSet::from([2, 4, 7, 8, 9]));

        let res = get_real_subsets(&candidates, tuple_len);
        assert_eq!(
            res,
            vec![
                BTreeSet::from(['a', 'b', 'c']),
            ]
        );
    }

    #[test]
    fn realsubset2() {
        let mut candidates = BTreeMap::new();
        let tuple_len = 2;

        candidates.insert('B', BTreeSet::from([5]));
        candidates.insert('C', BTreeSet::from([2, 3]));
        candidates.insert('D', BTreeSet::from([8, 9]));
        candidates.insert('E', BTreeSet::from([8, 9, 10, 11]));
        candidates.insert('F', BTreeSet::from([8, 9, 10, 11, 12, 13, 14, 15, 16]));
        candidates.insert(
            'K',
            BTreeSet::from([8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
        );
        candidates.insert('L', BTreeSet::from([8, 9]));
        candidates.insert('M', BTreeSet::from([4]));
        candidates.insert('H', BTreeSet::from([2, 3]));

        let res = get_real_subsets(&candidates, tuple_len);
        assert_eq!(
            res,
            vec![BTreeSet::from(['C', 'H']), BTreeSet::from(['D', 'L'])]
        );
    }
}
