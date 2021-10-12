use podaac::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	///////////////////////////////////
	//  Initiating the api wrapper  //
	//////////////////////////////////

	let podaac = Podaac::new();

	//////////////////////////
	//  Loading a dataset  //
	/////////////////////////

	let query = DatasetQuery {
		id: "PODAAC-AQR00-RFI01",
		short_name: "AQUARIUS_ANCILLARY_RFI_V1",
	};

	let dataset = podaac.dataset(&query).await?;

	println!("{:?}", dataset.personnel);

	/////////////////////////////////////
	//  Loading a dataset's variables  //
	/////////////////////////////////////

	let query = DatasetVariablesQuery {
		dataset_id: "PODAAC-ASOP2-25X01",
	};

	let dataset_variables = podaac.dataset_variables(&query).await?;

	println!("{:?}", dataset_variables);

	Ok(())
}
