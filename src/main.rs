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
    for u in 0..10 {
        dbg!(u);
        print_candidates(&candidates);

        reduce_by_max_value(&mut candidates, &row_sum, &translated_symbols);

        remove_hidden_tuples(&mut candidates, input);
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
        dbg!(format!("{}", frequencies.keys().join("")));
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
                .get_mut(&cs)
                .unwrap() // unwrap ok, since key definitely exists
                .retain(|&v| {
                    (frequencies[&cs] * (v as u32) + max <= row_sum[i])
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
///  Find hidden tuples
///
///  tuple length 1 is the special case where a symbol has been identified.
///
fn remove_hidden_tuples(candidates: &mut BTreeMap<char, BTreeSet<usize>>, input: &[(&str, u32)]) {
    // Start from highest value to remove largest tuple first
    for tuple_len in (1..input.len()).rev() {
        if candidates
            .values()
            .filter(|p| p.len() == tuple_len)
            .all_equal()
        {
            // Number of hidden tuple has to match the length of the tuple
            if candidates.values().filter(|p| p.len() == tuple_len).count() == tuple_len {
                let toberemoved = candidates
                    .values()
                    .find(|x| x.len() == tuple_len)
                    .unwrap() // unwrap ok, since checked in if above
                    .clone();
                // Remove tuple from those that do not exactly match the tuple
                for cands in candidates
                    .values_mut()
                    .filter(|p| toberemoved.symmetric_difference(p).count() != 0)
                {
                    for tbr in &toberemoved {
                        cands.remove(tbr);
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
        let solution = line
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

        let s = solve(&input, &solution);
        assert_eq!(s, "FOXTROTT");
    }
}
