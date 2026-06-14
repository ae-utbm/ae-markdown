use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
mod aemark {
    use pyo3::prelude::*;

    #[pyfunction]
    fn markdown(s: String) -> String {
        mark::markdown(&s)
    }
}
