use pyo3::prelude::*;
use smiles_with_selectors::random_generate_structures;

#[pyfunction]
fn rgs(start: &str, end: &str, duals: Vec<&str>, singles: Vec<&str>, duals_amount: usize) -> PyResult<String> {
    Ok(random_generate_structures(start, end, duals, singles, duals_amount).unwrap())
}

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn pysws(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rgs, m)?)?;
    Ok(())
}