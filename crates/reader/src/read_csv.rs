use crate::classify::classify_to_tracks;
use crate::models::{AdsbData, Track};
use std::fs::File;

pub fn read_csv(filename: &str) -> Result<Vec<Track>, Box<dyn std::error::Error>> {
    let mut csv_raw = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(File::open(filename)?);

    let mut adsb_data = Vec::new();
    for result in csv_raw.deserialize() {
        let record: AdsbData = result?;
        adsb_data.push(record);
    }

    Ok(classify_to_tracks(&adsb_data))
}
