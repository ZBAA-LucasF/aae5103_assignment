use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub icao_address: String,
    pub callsign: String,
    pub first_time: NaiveDateTime,
    pub last_time: NaiveDateTime,
    pub data: Vec<AdsbData>,
}

impl Track {
    pub(crate) fn new(callsign: &str, icao_address: &str, first_data: &AdsbData) -> Self {
        Track {
            callsign: callsign.to_string(),
            icao_address: icao_address.to_string(),
            data: vec![first_data.clone()],
            first_time: first_data.datetime,
            last_time: first_data.datetime,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AdsbData {
    #[serde(rename = "DateTime")]
    pub datetime: NaiveDateTime,
    #[serde(rename = "Register")]
    pub register: String,
    #[serde(rename = "IcaoAddress")]
    pub icao_address: String,
    #[serde(rename = "Callsign")]
    pub callsign: String,
    #[serde(rename = "Lat")]
    pub lat: f32,
    #[serde(rename = "Long")]
    pub long: f32,
    #[serde(rename = "Altitude")]
    pub altitude: i32,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "Heading")]
    pub heading: f32,
    #[serde(rename = "VerticalSpeed")]
    pub vertical_speed: f32,
}
