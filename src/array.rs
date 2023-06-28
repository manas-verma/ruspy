use num_complex::Complex;
use pyo3::{
    exceptions::{PyIndexError, PyTypeError},
    prelude::{pyclass, pyfunction, pymethods, PyAny, PyObject, PyResult, Python},
    types::{PyComplex, PyFloat, PyInt, PyList},
    PyErr, ToPyObject,
};

fn parse_real(item: &PyAny) -> PyResult<f64> {
    if let Ok(py_float) = item.downcast::<PyFloat>() {
        py_float.extract()
    } else if let Ok(py_int) = item.downcast::<PyInt>() {
        py_int.extract()
    } else {
        Err(PyTypeError::new_err("Invalid types in array"))
    }
}

fn parse_complex(item: &PyAny) -> PyResult<Complex<f64>> {
    if let Ok(py_complex) = item.downcast::<PyComplex>() {
        let real: f64 = py_complex.real();
        let imag: f64 = py_complex.imag();
        Ok(Complex::new(real, imag))
    } else if let Ok(py_float) = item.downcast::<PyFloat>() {
        let real: f64 = py_float.extract()?;
        Ok(Complex::new(real, 0.0))
    } else if let Ok(py_int) = item.downcast::<PyInt>() {
        let real: f64 = py_int.extract()?;
        Ok(Complex::new(real, 0.0))
    } else {
        Err(PyTypeError::new_err("Invalid types in array"))
    }
}

// fn safe_map(array: Array, map_fn: fn(Complex<f64>) -> Complex<f64>) -> Array {
//     let mut data = array.data.clone();
//     for i in 0..data.len() {
//         data[i] = map_fn(data[i]);
//     }
//     Array { data }
// }

#[pyclass]
#[derive(Clone)]
pub struct Array {
    pub data: Vec<Complex<f64>>,
}

#[pymethods]
impl Array {
    #[new]
    fn new(list: &PyList) -> Result<Self, PyErr> {
        let data_result = list
            .iter()
            .map(|item| parse_complex(item))
            .collect::<PyResult<Vec<Complex<f64>>>>();

        if let Ok(data) = data_result {
            Ok(Array { data })
        } else {
            Err(PyTypeError::new_err("Invalid types in array"))
        }
    }

    // fn FromPyObject(&self, py: Python, obj: &PyAny) -> PyResult<Self> {
    //     if let Ok(py_list) = obj.downcast::<PyList>() {
    //         Array::new(py_list)
    //     } else if let Ok(py_int) = obj.downcast::<PyInt>() {
    //         let data = vec![Complex::new(py_int.extract::<f64>()?, 0.0)];
    //         Ok(Array { data })
    //     } else if let Ok(py_float) = obj.downcast::<PyFloat>() {
    //         let data = vec![Complex::new(py_float.extract::<f64>()?, 0.0)];
    //         Ok(Array { data })
    //     } else if let Ok(py_complex) = obj.downcast::<PyComplex>() {
    //         let real: f64 = py_complex.real();
    //         let imag: f64 = py_complex.imag();
    //         let data = vec![Complex::new(real, imag)];
    //         Ok(Array { data })
    //     } else {
    //         Err(PyTypeError::new_err("Invalid types in array"))
    //     }
    // }

    // fn clone(&self) -> Self {
    //     Array {
    //         data: self.data.clone(),
    //     }
    // }

    fn __len__(&self) -> usize {
        self.data.len()
    }

    fn __getitem__(&self, py: Python, py_input: &PyInt) -> PyResult<PyObject> {
        if (py_input.extract::<i128>()).is_err() {
            return Err(PyTypeError::new_err("Index must be a positive integer"));
        }

        let py_index = py_input.extract::<i128>().unwrap();
        let len = self.data.len() as i128;
        if py_index >= len || py_index < -len {
            return Err(PyIndexError::new_err("Index out of range"));
        }

        let index_int = (py_index + len) % len;
        let index = index_int as usize;

        let complex = self.data[index];
        let py_complex = PyComplex::from_doubles(py, complex.re, complex.im);
        Ok(py_complex.to_object(py))
    }

    fn __mul__(&self, py_input: &PyAny) -> Result<Array, PyErr> {
        let scale = parse_complex(&py_input);
        if let Ok(scale) = scale {
            let data = self
                .data
                .iter()
                .map(|&x| x * scale)
                .collect::<Vec<Complex<f64>>>();
            Ok(Array { data })
        } else {
            Err(PyTypeError::new_err("Invalid types in array"))
        }
    }

    fn __rmul__(&self, py_input: &PyAny) -> Result<Array, PyErr> {
        self.__mul__(py_input)
    }

    fn __rdiv__(&self, py_input: &PyAny) -> Result<Array, PyErr> {
        let scale = parse_complex(&py_input);
        if let Ok(scale) = scale {
            let data = self
                .data
                .iter()
                .map(|&x| scale / x)
                .collect::<Vec<Complex<f64>>>();
            Ok(Array { data })
        } else {
            Err(PyTypeError::new_err("Invalid types in array"))
        }
    }

    fn __ldiv__(&self, py_input: &PyAny) -> Result<Array, PyErr> {
        let scale = parse_complex(&py_input);
        if let Ok(scale) = scale {
            let data = self
                .data
                .iter()
                .map(|&x| x / scale)
                .collect::<Vec<Complex<f64>>>();
            Ok(Array { data })
        } else {
            Err(PyTypeError::new_err("Invalid types in array"))
        }
    }

    fn __truediv__(&self, py_input: &PyAny) -> Result<Array, PyErr> {
        self.__ldiv__(py_input)
    }

    fn __add__(&self, py_input: &PyAny) -> Result<Array, PyErr> {
        let offset = parse_complex(&py_input);
        if let Ok(offset) = offset {
            let data = self
                .data
                .iter()
                .map(|&x| x + offset)
                .collect::<Vec<Complex<f64>>>();
            Ok(Array { data })
        } else {
            // Check if it's an array
            let array = py_input.extract::<Array>();
            if let Ok(array) = array {
                if array.data.len() != self.data.len() {
                    return Err(PyTypeError::new_err("Arrays must be the same length"));
                }
                let data = self
                    .data
                    .iter()
                    .zip(array.data.iter())
                    .map(|(&x, &y)| x + y)
                    .collect::<Vec<Complex<f64>>>();
                Ok(Array { data })
            } else {
                Err(PyTypeError::new_err("Invalid types in array"))
            }
        }
    }

    fn __sub__(&self, py_input: &PyAny) -> Result<Array, PyErr> {
        let offset = parse_complex(&py_input);
        if let Ok(offset) = offset {
            let data = self
                .data
                .iter()
                .map(|&x| x - offset)
                .collect::<Vec<Complex<f64>>>();
            Ok(Array { data })
        } else {
            // Check if it's an array
            let array = py_input.extract::<Array>();
            if let Ok(array) = array {
                if array.data.len() != self.data.len() {
                    return Err(PyTypeError::new_err("Arrays must be the same length"));
                }
                let data = self
                    .data
                    .iter()
                    .zip(array.data.iter())
                    .map(|(&x, &y)| x - y)
                    .collect::<Vec<Complex<f64>>>();
                Ok(Array { data })
            } else {
                Err(PyTypeError::new_err("Invalid types in array"))
            }
        }
    }

    fn __str__(&self) -> String {
        let mut string = String::from("[");
        for (i, item) in self.data.iter().enumerate() {
            string.push_str(&format!("{} + {}j", item.re, item.im));
            if i != self.data.len() - 1 {
                string.push_str(", ");
            }
        }
        string.push_str("]");
        string
    }

    fn __repr__(&self) -> String {
        let mut string = String::from("Array([");
        for (i, item) in self.data.iter().enumerate() {
            string.push_str(&format!("{} + {}j", item.re, item.im));
            if i != self.data.len() - 1 {
                string.push_str(", ");
            }
        }
        string.push_str("])");
        string
    }

    fn __list__(&self, py: Python) -> PyResult<PyObject> {
        let list = PyList::empty(py);
        for item in self.data.iter() {
            let py_complex = PyComplex::from_doubles(py, item.re, item.im);
            list.append(py_complex)?;
        }
        Ok(list.to_object(py))
    }

    fn __pow__(&self, py_input: &PyAny, _py: &PyAny) -> Result<Array, PyErr> {
        let power = parse_real(&py_input);
        if let Ok(power) = power {
            let data = self
                .data
                .iter()
                .map(|&x| x.powf(power))
                .collect::<Vec<Complex<f64>>>();
            Ok(Array { data })
        } else {
            Err(PyTypeError::new_err("Invalid types in array"))
        }
    }
}

#[pyfunction]
pub fn array(_py: Python, list: &PyList) -> PyResult<Array> {
    Array::new(list)
}

#[pyfunction]
pub fn zeros(_py: Python, size: usize) -> PyResult<Array> {
    let data = vec![Complex::new(0.0, 0.0); size];
    Ok(Array { data })
}

#[pyfunction]
pub fn linspace(_py: Python, start: f64, stop: f64, num: usize) -> PyResult<Array> {
    let data = (0..num)
        .map(|i| {
            Complex::new(
                start + (stop - start) * (i as f64) / ((num - 1) as f64),
                0.0,
            )
        })
        .collect::<Vec<Complex<f64>>>();
    Ok(Array { data })
}

#[pyfunction]
pub fn pad(_py: Python, array: &Array) -> PyResult<Array> {
    let mut data = array.data.clone();
    let n = data.len();
    let p = 2usize.pow((n as f64).log2().ceil() as u32);
    data.resize(p, Complex::new(0.0, 0.0));
    Ok(Array { data })
}
