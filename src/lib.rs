use pyo3::prelude::*;
use smiles_with_selectors::workspace::Workspace;
use structure_generator::{random_generate_structure, SmilesStackfulSelector};
mod structure_generator;
// use structure_generator::random_generate_structures;

#[pyfunction]
fn rgs(
    start: &str,
    replacers: Vec<(isize, usize, Vec<&str>, &str, &str, &str)>,
) -> PyResult<String> {
    Ok(random_generate_structure(start, replacers).unwrap())
}

#[pyfunction]
fn ligand_index(sws: &str) -> PyResult<Vec<usize>> {
    let mut ws = Workspace::new();
    let root = ws.add_structure(sws).unwrap();
    let nodes = ws.filter_with_selector(root, "L");
    Ok(nodes.iter()
        .map(|ni| ni.index())
        .collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pysws(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rgs, m)?)?;
    m.add_function(wrap_pyfunction!(ligand_index, m)?)?;
    Ok(())
}
