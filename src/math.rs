use crate::array::Array;
use pyo3::prelude::{pyfunction, PyResult, Python};

#[pyfunction]
pub fn sin(_py: Python, array: &Array) -> PyResult<Array> {
    let mut data = array.data.clone();
    for i in 0..data.len() {
        data[i] = data[i].sin();
    }
    Ok(Array { data })
}

#[pyfunction]
pub fn cos(_py: Python, array: &Array) -> PyResult<Array> {
    let mut data = array.data.clone();
    for i in 0..data.len() {
        data[i] = data[i].cos();
    }
    Ok(Array { data })
}
