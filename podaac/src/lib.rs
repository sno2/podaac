mod error;
mod models;

pub use error::PodaacError;
pub use models::*;
use reqwest::StatusCode;
use serde_xml_rs::from_str;

#[cfg(feature = "python")]
pub use dict_derive::IntoPyObject;

#[derive(Debug, Clone)]
/// This is cheap to clone because it just leverages [`reqwest::Client`] which
/// is also cheap to clone.
pub struct Podaac {
	client: reqwest::Client,
}

#[derive(Debug)]
pub struct DatasetQuery<'a> {
	pub id: &'a str,
	pub short_name: &'a str,
}

#[derive(Debug)]
pub enum DatasetStatus {
	Open,
	Preview,
	Simulated,
	Retired,
}

impl DatasetStatus {
	pub fn as_str(&self) -> &'static str {
		match self {
			Self::Open => "OPEN",
			Self::Preview => "PREVIEW",
			Self::Simulated => "SIMULATED",
			Self::Retired => "RETIRED",
		}
	}
}

#[derive(Debug)]
pub struct DatasetSearchQuery<'a> {
	pub keyword: &'a str,
	pub start_time: &'a str,
	pub end_time: &'a str,
	pub start_index: &'a str,
	pub items_per_page: u32,
	pub dataset_id: &'a str,
	pub short_name: &'a str,
	pub instrument: &'a str,
	pub satellite: &'a str,
	pub status: DatasetStatus,
	pub process_level: &'a str,
	pub bbox: &'a (f32, f32, f32, f32),
	pub full: bool,
}

#[derive(Debug)]
pub struct DatasetVariablesQuery<'a> {
	pub dataset_id: &'a str,
}

impl Podaac {
	/// Creates a new instance of the [`Podaac`] API wrapper.
	pub fn new() -> Self {
		Self::with_client(reqwest::Client::new())
	}

	/// Creates a new [`Podaac`] instance with the given [`reqwest::Client`].
	#[inline]
	pub const fn with_client(client: reqwest::Client) -> Self {
		Self { client }
	}

	fn bytes_to_str(bytes: &bytes::Bytes) -> Result<&str, PodaacError> {
		std::str::from_utf8(bytes).map_err(|_| PodaacError::InvalidUtf8)
	}

	/// Gets the metadata of a dataset that matches the given query.
	pub async fn dataset(&self, query: &DatasetQuery<'_>) -> Result<Dataset, PodaacError> {
		let req = self
			.client
			.get("https://podaac.jpl.nasa.gov/ws/metadata/dataset")
			.query(&[
				("datasetId", query.id),
				("shortName", query.short_name),
				("format", "gcmd"),
			])
			.build()?;

		let res = self.client.execute(req).await?;

		match res.status() {
			StatusCode::OK => (),
			status => return Err(PodaacError::InvalidStatus(status)),
		}

		let bytes = res.bytes().await?;
		let text = Self::bytes_to_str(&bytes)?;

		Ok(from_str::<Dataset>(text)?)
	}

	pub async fn search_datasets(&self, query: &DatasetSearchQuery<'_>) -> Result<(), PodaacError> {
		Ok(())
	}

	/// Gets information about a dataset's variables. This method requires the id
	/// within your [`DatasetVariablesQuery`].
	///
	/// ## Examples
	///
	/// ```rust no-run
	/// let podaac = Podaac::new();
	///
	///	let query = DatasetVariablesQuery {
	/// 	dataset_id: "PODAAC-ASOP2-25X01"
	/// };
	///
	/// let dataset_variables = podaac.dataset_variables(&query).await?;
	/// ```
	pub async fn dataset_variables(
		&self,
		query: &DatasetVariablesQuery<'_>,
	) -> Result<Variable, PodaacError> {
		let req = self
			.client
			.get("https://podaac.jpl.nasa.gov/ws/dataset/variables")
			.header("accept", "application/json")
			.query(&[("datasetId", query.dataset_id)])
			.build()?;

		let res = self.client.execute(req).await?;

		Ok(res.json().await?)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
