use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    // only add functionality in this library related to converting rust lib responses into necessary python formats!
    Ok(format!("Sum as string == {}!", rustlib::add(a, b)))
}

#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(rustlib::greeting(name))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustpylib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}
