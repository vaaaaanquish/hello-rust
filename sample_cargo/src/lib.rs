use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::f64::consts::PI;

mod foo;

#[pyfunction]
fn pi_times( n: usize ) -> PyResult<Vec<f64>> {
    Ok(
        (0..n).map(|i| i as f64 * PI).collect()
    )
}

#[pyfunction]
fn add_str(s: String) -> PyResult<String> {
	Ok(
		s+"??"
	)
}

#[pyfunction]
fn print_str(s: String) -> PyResult<()>{
    println!("input:{}",s);
    Ok(())
}

#[pyfunction]
fn print_list(l: Vec<i32>) -> PyResult<Vec<i32>>{
    let mut vec: Vec<i32> = Vec::new();

    for x in l{
        println!("{}", x);
        vec.push(x);
    }
    vec.push(3);
    vec.push(3);
    vec.push(3);
    Ok(vec)
}

#[pyfunction]
fn print_mod(s: String) -> PyResult<()>{
    foo::bar::hello(s);
    Ok(())
}

mod mods;

#[pyfunction]
fn print_dir(s: String) -> PyResult<()>{
    mods::hoge::bar::hello(s);
    Ok(())
}

#[pymodule]
fn rustpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!( pi_times ))?;
    m.add_wrapped(wrap_pyfunction!( add_str ))?;
    m.add_wrapped(wrap_pyfunction!( print_str ))?;
    m.add_wrapped(wrap_pyfunction!( print_list ))?;
    m.add_wrapped(wrap_pyfunction!( print_mod ))?;
    m.add_wrapped(wrap_pyfunction!( print_dir ))?;
    Ok(())
}

