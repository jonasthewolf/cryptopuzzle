use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
    num::ParseIntError,
};

///
/// Solve the crypto puzzle
///
///
fn solve(input: &[(&str, u32)]) -> BTreeMap<char, u32> {
    // Build symbol table
    let mut symbols = BTreeSet::new();
    input.iter().for_each(|(word, _)| {
        word.to_uppercase().chars().for_each(|c| {
            symbols.insert(c);
        })
    });

    let sym_len = symbols.len();
    let mut candidates = BTreeMap::new();
    symbols.iter().for_each(|&s| {
        candidates.insert(s, BTreeSet::from_iter(1..=sym_len));
    });

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

    // Do until all candidates have only single value left.
    while !candidates.values().all(|x| x.len() == 1) {
        // Optimization would be if all numbers from "solution" array are resolved.
        // for u in 0..12 {
        reduce_by_max_value(&mut candidates, &row_sum, &translated_symbols);

        remove_hidden_tuples(&mut candidates, input);

        remove_singles(&mut candidates);
    }

    candidates
        .iter()
        .map(|(c, s)| (*c, (*s.first().unwrap()) as u32)) // unwrap ok, since while above would not have terminated otherwise
        .collect::<BTreeMap<char, u32>>()
}

///
/// Translate remaining candidates into string
///
///
fn translate_into_symbols(candidates: &BTreeMap<char, u32>, solution: &[u32]) -> String {
    // Convert list of numbers from solution to characters
    solution
        .iter()
        .map(|&x| {
            candidates
                .iter()
                .find_map(|(c, &num)| if num == x { Some(*c) } else { None })
                .unwrap() // unwrap ok, since we expect that puzzle is solved.
        })
        .collect::<String>()
        .to_uppercase()
}

///
/// Remove identified values (i.e. a symbol has only a single remaining value) from other symbols candidates
///
///
fn remove_singles(candidates: &mut BTreeMap<char, BTreeSet<usize>>) {
    let mut to_be_removed = vec![];
    candidates.iter().for_each(|(cand_char, cand_set)| {
        if cand_set.len() == 1 {
            to_be_removed.push((*cand_char, *cand_set.first().unwrap())); // unwrap ok, since if there are empty sets, the whole algorithm is not working correctly anyway
        }
    });
    for (tbr_char, tbr_value) in to_be_removed {
        candidates.iter_mut().for_each(|(c, v)| {
            if *c != tbr_char {
                v.remove(&tbr_value);
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
                        .filter(|f| !selectedcands.values().any(|&v| v == **f))
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
                        || selectedcands.values().any(|&f| v == f)
                });
        }
    }
}

///
/// Print current candidates list
///
/// Debugging purposes only
///
fn _print_candidates(candidates: &BTreeMap<char, BTreeSet<usize>>) {
    let mut sorted_candidates = Vec::from_iter(candidates);
    sorted_candidates.sort_by(|a, b| Ord::cmp(a.0, b.0));
    for (c, cs) in sorted_candidates {
        println!(
            "{c} : {}",
            cs.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

///
/// Get real subsets in candidates with at most length of tuple_len
///
///
fn get_real_subsets(
    candidates: &BTreeMap<char, BTreeSet<usize>>,
    tuple_len: usize,
) -> Vec<BTreeSet<char>> {
    let tuples = candidates
        .iter()
        .filter(|(_, p)| p.len() <= tuple_len)
        .collect::<Vec<_>>();
    let mut result: Vec<BTreeSet<char>> = Vec::new();

    for (&first_char, first_set) in &tuples {
        for (&second_char, second_set) in &tuples {
            if second_char != first_char
                && ((second_set.is_superset(first_set) && first_set.is_subset(second_set))
                    || (first_set.is_superset(second_set) && second_set.is_subset(first_set)))
            {
                let mut new_set = true;
                for set in result.iter_mut() {
                    if set.contains(&first_char) || set.contains(&second_char) {
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
    result
}

///
/// Find hidden tuples
///
/// Tuple length 1 is the special case where a symbol has been identified.
/// This is treated separately.
///
fn remove_hidden_tuples(candidates: &mut BTreeMap<char, BTreeSet<usize>>, input: &[(&str, u32)]) {
    // Start from highest value to remove largest tuple first
    for tuple_len in (2..input.len() - 1).rev() {
        // -1 because it does not make sense to start with all tuples
        let subsets = get_real_subsets(candidates, tuple_len);
        for subset in subsets {
            // Number of symbols has to match the number of tuples to be a hidden tuple
            if subset.len() == tuple_len {
                let sub = subset
                    .iter()
                    .flat_map(|x| candidates[x].iter().copied())
                    .collect::<BTreeSet<_>>();
                for (cand_char, cand_set) in candidates.iter_mut() {
                    if !subset.contains(cand_char) {
                        cand_set.retain(|v| !sub.contains(v));
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
            .split_whitespace()
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;
        let mut input = vec![];
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }
            let inp = line.split_whitespace().collect::<Vec<_>>();
            input.push((inp[0].to_owned(), inp[1].parse::<u32>()?));
        }
        let input = input
            .iter()
            .map(|(a, b)| (a.as_str(), *b))
            .collect::<Vec<_>>();
        let sol = solve(&input);
        println!("{}", translate_into_symbols(&sol, &solution));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{
        collections::{BTreeMap, BTreeSet},
        io::Write,
        process::{Command, Stdio},
    };

    use crate::{get_real_subsets, solve, translate_into_symbols};

    #[test]
    fn backbackpuzzle1() -> Result<(), std::io::Error> {
        let inputtxt = concat!(
            "9 16 7 2 6 18 6 22\n",
            "BOOGIE  58\n",
            "BOSSANOVA  66\n",
            "CHACHACHA  18\n",
            "CHARLESTON  76\n",
            "FLAMENCO  62\n",
            "FOXTROTT  102\n",
            "JIVE  69\n",
            "LAMBADA  29\n",
            "MAMBO  20\n",
            "PASODOBLE  63\n",
            "QUICKSTEP  118\n",
            "ROCKNROLL  88\n",
            "RUMBA  43\n",
            "SAMBA  18\n",
            "SHIMMY  59\n",
            "SIRTAKI  83\n",
            "TANGO  47\n",
            "TWIST  68\n",
            "WALZER  80\n",
            "\n"
        );

        let mut child = Command::new("cargo")
            .args(["run", "--"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().unwrap();
        std::thread::spawn(move || {
            stdin.write_all(inputtxt.as_bytes()).unwrap();
        });

        let output = child.wait_with_output()?;
        assert_eq!(String::from_utf8_lossy(&output.stdout), "DISCOFOX\n");
        Ok(())
    }

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

        let sol = solve(&input);
        let sol_string = translate_into_symbols(&sol, &solution);
        assert_eq!(sol_string, "DISCOFOX");

        let sol_exp = BTreeMap::from([
            ('A', 1),
            ('B', 5),
            ('C', 2),
            ('D', 9),
            ('E', 10),
            ('F', 18),
            ('G', 15),
            ('H', 3),
            ('I', 16),
            ('J', 23),
            ('K', 17),
            ('L', 8),
            ('M', 4),
            ('N', 13),
            ('O', 6),
            ('P', 11),
            ('Q', 24),
            ('R', 14),
            ('S', 7),
            ('T', 12),
            ('U', 19),
            ('V', 20),
            ('W', 21),
            ('X', 22),
            ('Y', 25),
            ('Z', 26),
        ]);
        assert_eq!(sol, sol_exp);
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
        assert_eq!(res, vec![BTreeSet::from(['a', 'b', 'c']),]);
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
