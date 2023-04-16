
use std::collections::{BTreeSet, BTreeMap};

use mathru::algebra::linear::{Vector, Matrix};
use mathru::algebra::linear::matrix::{Solve, Transpose};

fn solve(input: Vec<(&str, u32)>, solution: &[u32]) -> String {
    // Build symbol table
    let mut symbols = BTreeSet::new();
    input.iter().for_each(|(i,_)| i.to_uppercase().chars().for_each(|c| { symbols.insert(c); } ));

    let symbol_table = symbols.iter().enumerate().map(|(i,c)| (*c,i)).collect::<BTreeMap<char,usize>>();

    dbg!(&symbols.len());

    // B
    let b = input.iter().map(|(_,b)| *b as f64).collect::<Vec<f64>>();

    // Setup A
    let mut a = vec![];
    for (i,_) in input {
        let mut a_in = (0..symbols.len()).map(|_| 0).collect::<Vec<_>>();
        for c in i.to_uppercase().chars() {
            a_in[symbol_table[&c]] += 1;
        }
        a.push(a_in);
    }
    // Debug
    dbg!(a.len());
    dbg!(a[0].len());
    dbg!(b.len());
    assert_eq!(a.len(), b.len());
    let a0_len = a[0].len();
    assert!(a.iter().all(|x| x.len() == a0_len));

    // Make matrix A
    let mut a_matrix: Matrix<f64> = Matrix::zero(a.len(),a[0].len());
    for row in 0..a.len() {
        for col in 0..a0_len {
            a_matrix[[row,col]] = a[row][col] as f64;
        }
    }
    dbg!(&a_matrix);
    let b_vec: Vector<f64> = Vector::new_column(b);
    dbg!(&b_vec);

    let x = a_matrix.solve(&b_vec);
    dbg!(&x);

    // FIXME use solution X
    solution.iter().map(|&sym| symbol_table.iter().find_map(|(&c, &s)| if s == sym.try_into().unwrap() { Some(c)} else {None}).unwrap()).collect::<String>().to_uppercase()

} 



fn main() {

}


#[cfg(test)]
mod test {
    use crate::solve;


    #[test]
    fn test1() {
        let input = vec![("BOOGIE", 58), 
        ("BOSSANOVA",66),
        ("CHACHACHA",18),
        ("CHARLESTON",76),
        ("FLAMENCO",62),
        ("FOXTROTT",102),
        ("JIVE",69),
        ("LAMBADA",29),
        ("MAMBO",20),
        ("PASODOBLE",63),
        ("QUICKSTEP",118),
        ("ROCKNROLL",88),
        ("RUMBA",43),
        ("SAMBA",18),
        ("SHIMMY",59),
        ("SIRTAKI",83),
        ("TANGO",47),
        ("TWIST",68),
        ("WALZER",80)];

        let solution = [9,16,7,2,6,8,16,6,22];

        let s = solve(input, &solution);
        assert_eq!(s, "FOXTROTT");

    }
}