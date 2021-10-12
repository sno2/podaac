#[cfg(feature = "python")]
use dict_derive::IntoPyObject;
#[cfg(feature = "python")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Personel {
	#[serde(rename = "Role")]
	pub role: Option<String>,
	#[serde(rename = "First_Name")]
	pub first_name: Option<String>,
	#[serde(rename = "Last_Name")]
	pub last_name: Option<String>,
	#[serde(rename = "Phone")]
	pub phone: Option<String>,
	#[serde(rename = "Fax")]
	pub fax: Option<String>,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
	#[serde(rename = "Category")]
	pub category: String,
	#[serde(rename = "Topic")]
	pub topic: String,
	#[serde(rename = "Term")]
	pub term: String,
	#[serde(rename = "Variable_Level_1")]
	pub variable_level_1: String,
	#[serde(rename = "Detailed_Variable")]
	pub detailed_variable: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct NameInfo {
	#[serde(rename = "Short_Name")]
	pub short_name: Option<String>,
	#[serde(rename = "Long_Name")]
	pub long_name: Option<String>,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct TemporalCoverageInfo {
	#[serde(rename = "Start_Date")]
	pub start_date: String,
	#[serde(rename = "Stop_Date")]
	pub stop_date: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SpacialCoverageInfo {
	#[serde(rename = "Southernmost_Latitude")]
	pub southernmost_latitude: i64,
	#[serde(rename = "Northernmost_Latitude")]
	pub northernmost_latitude: i64,
	#[serde(rename = "Westernmost_Longitude")]
	pub westernmost_longitude: i64,
	#[serde(rename = "Easternmost_Longitude")]
	pub easternmost_longitude: i64,
	#[serde(rename = "Maximum_Altitude")]
	pub maximum_altitude: i64,
	#[serde(rename = "Minimum_Altitude")]
	pub minimum_altitude: i64,
	#[serde(rename = "Maximum_Depth")]
	pub maximum_depth: i64,
	#[serde(rename = "Minimum_Depth")]
	pub minimum_depth: i64,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationInfo {
	#[serde(rename = "Location_Category")]
	pub location_category: String,
	#[serde(rename = "Location_Type")]
	pub location_type: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataResolutionInfo {
	#[serde(rename = "Latitude_Resolution")]
	pub latitude_resolution: String,
	#[serde(rename = "Longitude_Resolution")]
	pub longitude_resolution: String,
	#[serde(rename = "Horizontal_ResolutionRange")]
	pub horizontal_resolution_range: Option<String>,
	#[serde(rename = "Temporal_Resolution")]
	pub temporal_resolution: String,
	#[serde(rename = "Temporal_ResolutionRange")]
	pub temporal_resolution_range: Option<String>,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataCenter {
	#[serde(rename = "Data_Center_Name")]
	pub name: NameInfo,
	#[serde(rename = "Data_Center_URL")]
	pub url: NameInfo,
	#[serde(rename = "Data_Set_ID")]
	pub data_set_id: Option<NameInfo>,
	#[serde(rename = "Personnel")]
	pub personnel: Personel,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct DatasetSummary {
	#[serde(rename = "Abstract")]
	pub r#abstract: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct ShortNameInfo {
	#[serde(rename = "Short_Name")]
	pub short_name: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Citation {
	#[serde(rename = "Dataset_Creator")]
	pub creator: String,
	#[serde(rename = "Dataset_Title")]
	pub title: String,
	#[serde(rename = "Dataset_Series_Name")]
	pub series_name: String,
	#[serde(rename = "Dataset_Release_Date")]
	pub release_date: String,
	#[serde(rename = "Dataset_Release_Place")]
	pub release_place: String,
	#[serde(rename = "Dataset_Publisher")]
	pub publisher: String,
	#[serde(rename = "Version")]
	pub version: String,
	#[serde(rename = "Other_Citation_Details")]
	pub other_citation_details: String,
	#[serde(rename = "Online_Resource")]
	pub online_resource: String,
}

#[cfg_attr(feature = "python", derive(IntoPyObject))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Dataset {
	#[serde(rename = "Entry_ID")]
	pub id: String,
	#[serde(rename = "Entry_Title")]
	pub title: String,
	#[serde(rename = "Data_Set_Citation")]
	pub citation: Citation,
	#[serde(rename = "Personnel")]
	pub personnel: Vec<Personel>,
	#[serde(rename = "Parameters")]
	pub parameters: Personel,
	#[serde(rename = "ISO_Topic_Category")]
	pub iso_topic_categories: Vec<String>,
	#[serde(rename = "Sensor_Name")]
	pub sensor_names: Vec<NameInfo>,
	#[serde(rename = "Source_Name")]
	pub source_name: NameInfo,
	#[serde(rename = "Temporal_Coverage")]
	pub temporal_coverage: TemporalCoverageInfo,
	#[serde(rename = "Location")]
	pub location: LocationInfo,
	#[serde(rename = "Data_Resolution")]
	pub resolution: DataResolutionInfo,
	#[serde(rename = "Project")]
	pub project: NameInfo,
	#[serde(rename = "Access_Constraints")]
	pub access_constraints: String,
	#[serde(rename = "Use_Constraints")]
	pub use_constraints: String,
	#[serde(rename = "Data_Set_Language")]
	pub data_set_language: String,
	#[serde(rename = "Originating_Center")]
	pub originating_center: String,
	#[serde(rename = "Data_Center")]
	pub data_center: DataCenter,
	#[serde(rename = "Reference")]
	pub reference: Option<String>,
	#[serde(rename = "Summary")]
	pub summary: DatasetSummary,
	#[serde(rename = "IDN_Node")]
	pub idn_node: ShortNameInfo,
	#[serde(rename = "Metadata_Name")]
	pub metadata_name: String,
	#[serde(rename = "Metadata_Version")]
	pub metadata_version: String,
	#[serde(rename = "DIF_Creation_Date")]
	pub dif_creation_date: String,
	#[serde(rename = "Last_DIF_Creation_Date")]
	pub last_dif_revision_date: Option<String>,
	#[serde(rename = "DIF_Revision_History")]
	pub dif_revision_history: String,
}
