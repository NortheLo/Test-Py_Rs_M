use pyo3::prelude::*;
use std::fs::File;
use std::io::{Error, Read, Write, ErrorKind};

#[pyclass(name = "Header")]
#[derive(Debug, Clone)] // Enable printing & copying
pub struct Header {
    #[pyo3(get, set)] // Expose to Python
    counter: u32,
    
    #[pyo3(get, set)]
    data: Vec<f32>,  // Python doesn't handle fixed-size arrays well, so use Vec<f32>
}

#[pymethods]
impl Header {
    #[new]  
    fn new(counter: u32, data: Vec<f32>) -> Self {
        Header { counter, data }
    }
}

#[pyfunction]
pub fn write_bin(path: String, hdr: Header) -> Result<i32, std::io::Error> {
    let mut file = File::create(path)?;

    // Write first struct
    file.write_all(&hdr.counter.to_le_bytes())?;
    for &val in &hdr.data {
        file.write_all(&val.to_le_bytes())?;
    }

    file.flush()?; // Ensure all data is written

    return Ok(0);
}

#[pyfunction]
pub fn read_bin(path: String) -> Result<Header, std::io::Error> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    
    // Read entire file into buffer
    f.read_to_end(&mut buffer)?;

    // Ensure file has at least 4 bytes (for counter)
    if buffer.len() < 4 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "File too small"));
    }

    // Extract counter (first 4 bytes)
    let counter = u32::from_le_bytes(buffer[0..4].try_into().unwrap());

    // Extract f32 values from remaining bytes
    let mut data = Vec::new();
    for chunk in buffer[4..].chunks_exact(4) {
        let val = f32::from_le_bytes(chunk.try_into().unwrap());
        data.push(val);
    }

    // Return parsed Header struct
    Ok(Header { counter, data })

}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
pub fn bin_fil(module: &Bound<'_, PyModule>) -> Result<(), pyo3::PyErr> {
    module.add_class::<Header>()?;
    module.add_function(wrap_pyfunction!(write_bin, module)?)?;
    module.add_function(wrap_pyfunction!(read_bin, module)?)?;
    module.add_function(wrap_pyfunction!(sum_as_string, module)?)?;

    Ok(())
}