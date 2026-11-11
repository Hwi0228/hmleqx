use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyModule;

/// Convert a Hancom equation script to LaTeX using the Rust parser.
#[pyfunction]
fn to_latex(source: &str) -> PyResult<String> {
    hmleq::eq_to_latex(source).map_err(|error| PyValueError::new_err(error.to_string()))
}

/// Parse a Hancom equation script and return its serde AST as Python objects.
#[pyfunction]
fn parse(py: Python<'_>, source: &str) -> PyResult<Py<PyAny>> {
    let ast = hmleq::parse(source).map_err(|error| PyValueError::new_err(error.to_string()))?;
    let encoded = serde_json::to_string(&ast)
        .map_err(|error| PyRuntimeError::new_err(format!("failed to serialize AST: {error}")))?;

    // Use Python's standard-library JSON decoder so callers receive ordinary
    // dict/list/scalar values rather than a JSON string or a custom wrapper.
    let json = PyModule::import(py, "json")?;
    Ok(json.call_method1("loads", (encoded,))?.unbind())
}

/// Version of the native Python extension.
#[pyfunction]
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn _core(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(to_latex, module)?)?;
    module.add_function(wrap_pyfunction!(parse, module)?)?;
    module.add_function(wrap_pyfunction!(version, module)?)?;
    Ok(())
}
