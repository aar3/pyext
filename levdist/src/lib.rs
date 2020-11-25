use std::collections::HashMap;

use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[derive(Debug)]
struct Weight(u64, u64, u64);

#[derive(Debug)]
struct CharWeight(char, Weight);

#[pyfunction]
fn levenshtein_distance(s: &str, t: &str) -> PyResult<u64> {
    let sbytes = s.as_bytes();
    let tbytes = t.as_bytes();

    let ncols = sbytes.len() + 1;
    let nrows = tbytes.len() + 1;

    let tups: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .as_bytes()
        .into_iter()
        .map(|ch| (*ch, Weight(1, 1, 1)))
        .collect();

    let weights: HashMap<u8, Weight> = tups.into_iter().collect();

    let mut matrix: ArrayBase<OwnedRepr<u64>, Dim<[usize; 2]>> = Array2::zeros((nrows, ncols));

    for row in 1..nrows {
        matrix[[row, 0]] = matrix[[row, 0]] + weights.get(&sbytes[row - 1]).unwrap().0;
    }

    for col in 1..ncols {
        matrix[[0, col]] = matrix[[0, col - 1]] + weights.get(&sbytes[col - 1]).unwrap().1;
    }

    for col in 1..ncols {
        for row in 1..nrows {
            let deletes = weights.get(&sbytes[row - 1]).unwrap().0;
            let inserts = weights.get(&tbytes[col - 1]).unwrap().1;

            let mut subs = std::cmp::max(
                weights.get(&sbytes[row - 1]).unwrap().2,
                weights.get(&tbytes[col - 1]).unwrap().2,
            );

            if sbytes[row - 1] == tbytes[col - 1] {
                subs = 0;
            }

            matrix[[row, col]] = std::cmp::min(
                std::cmp::min(
                    matrix[[row - 1, col]] + deletes,
                    matrix[[row, col - 1]] + inserts,
                ),
                matrix[[row - 1, col - 1]] + subs,
            )
        }
    }

    Ok(matrix[[row, col]])
}

#[pymodule]
fn levdist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(levenshtein_distance))?;
    Ok(())
}
