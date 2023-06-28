use pyo3::{
    prelude::{pymodule, PyModule, PyResult, Python},
    wrap_pyfunction,
};
mod array;
use array::{array as array_fn, zeros, linspace, pad, Array};
mod fft;
use fft::{fft_basic as fft_basic_fn, fft as fft_fn};
mod math;
use math::{sin, cos};
mod test;

#[pymodule]
fn ruspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Array>()?;
    m.add_function(wrap_pyfunction!(array_fn, m)?)?;
    m.add_function(wrap_pyfunction!(fft_basic_fn, m)?)?;
    m.add_function(wrap_pyfunction!(fft_fn, m)?)?;
    m.add_function(wrap_pyfunction!(zeros, m)?)?;
    m.add_function(wrap_pyfunction!(linspace, m)?)?;
    m.add_function(wrap_pyfunction!(pad, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    Ok(())
}
