pub mod data;
pub mod fburpp;
pub mod read_write;

pub use fburpp::execute;


use pyo3::prelude::*;


#[pyfunction]
fn fburpp_execute(json: String) -> PyResult<(), > {
    execute(&json).unwrap();
    Ok(())
}

#[pymodule]
fn fburpp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fburpp_execute, m)?)?;
    Ok(())
}
