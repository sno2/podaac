#[cfg(feature = "python")]
use dict_derive::IntoPyObject;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct VariableFootprint {
	pub b: String,
	pub s1: String,
	pub strategy: String,
	pub t: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct VariableImage {
	pub ppd: i64,
	pub res: i64,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageVariable {
	pub id: String,
	pub max: String,
	pub min: String,
	pub palette: String,
	pub title: String,
	pub units: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct VariableTiles {
	pub steps: Vec<i64>,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
	pub footprint: VariableFootprint,
	pub image: VariableImage,
	#[serde(rename = "imgVariables")]
	pub image_variables: Vec<ImageVariable>,
	pub is_360: bool,
	pub lat_var: String,
	pub lon_var: String,
	pub tiles: VariableTiles,
	pub variables: Vec<String>,
}
