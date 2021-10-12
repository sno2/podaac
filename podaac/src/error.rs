use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PodaacError {
	#[error("failed to build request")]
	RequestBuild,
	#[error("invalid response status: {0}")]
	InvalidStatus(StatusCode),
	#[error("reqwest error: {0}")]
	Reqwest(#[from] reqwest::Error),
	#[error("parse error: {0}")]
	ParseError(#[from] serde_xml_rs::Error),
	#[error("failed to parse invalid utf-8")]
	InvalidUtf8,
}
