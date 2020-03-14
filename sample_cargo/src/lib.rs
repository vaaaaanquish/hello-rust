use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::f64::consts::PI;

#[pyfunction]
fn pi_times( n: usize ) -> PyResult<Vec<f64>> {
    Ok(
        (0..n).map(|i| i as f64 * PI).collect()
    )
}

#[pymodule]
fn rustpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!( pi_times ))?;
    Ok(())
}
