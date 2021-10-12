use podaac::{DatasetQuery, DatasetVariablesQuery, Podaac as _Podaac};
use pyo3::prelude::*;

use std::sync::Arc;

#[pyclass]
/// The struct for interacting with the PO.DAAC web APIs.
pub struct Podaac {
	podaac: Arc<_Podaac>,
}

#[pymethods]
impl Podaac {
	#[new]
	pub fn new() -> Self {
		Self {
			podaac: Arc::new(_Podaac::new()),
		}
	}

	/// Gets the metadata about a dataset with the given id and short name.
	pub fn dataset<'p>(
		&self,
		py: Python<'p>,
		id: String,
		short_name: String,
	) -> PyResult<&'p PyAny> {
		let podaac = self.podaac.clone();
		pyo3_asyncio::tokio::future_into_py(py, async move {
			let query = DatasetQuery {
				id: id.as_str(),
				short_name: short_name.as_str(),
			};

			let dataset = match podaac.dataset(&query).await {
				Ok(ds) => ds,
				Err(err) => {
					return Err(pyo3::exceptions::PyConnectionError::new_err(format!(
						"{:?}",
						err
					)));
				}
			};

			Ok(Python::with_gil(|py| dataset.into_py(py)))
		})
	}

	pub fn dataset_variables<'p>(&self, py: Python<'p>, id: String) -> PyResult<&'p PyAny> {
		let podaac = self.podaac.clone();
		pyo3_asyncio::tokio::future_into_py(py, async move {
			let query = DatasetVariablesQuery {
				dataset_id: id.as_str(),
			};

			let variables = match podaac.dataset_variables(&query).await {
				Ok(ds) => ds,
				Err(err) => {
					return Err(pyo3::exceptions::PyConnectionError::new_err(format!(
						"{:?}",
						err
					)));
				}
			};

			Ok(Python::with_gil(|py| variables.into_py(py)))
		})
	}
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn podaac_py(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<Podaac>()?;

	Ok(())
}
