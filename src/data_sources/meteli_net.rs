// Implement mod.rs -> DataSource trait for https://meteli.net

use crate::data_sources::Gig;

use super::*;
use std::fmt::{Debug, Error};

use serde_derive::{Deserialize, Serialize};

pub type MeteliNetPageJson = Vec<MeteliNetGig>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeteliNetGig {
    pub id: i64,
    pub name: String,
    pub date: String,
    pub location: String,
    #[serde(rename = "location_url")]
    pub location_url: String,
    pub latitude: String,
    pub longitude: String,
    pub link: String,
}

pub struct MeteliNet;

impl DataSource for MeteliNet {
    fn get_gigs(&self, band_name: String) -> Result<Events, Error> {
        let endpoint = format!("https://www.meteli.net/tapahtumahaku?q={}", band_name);

        let request = reqwest::blocking::get(endpoint).unwrap().text().unwrap();
        let matching_line = request
            .split("\n")
            .filter(|x| x.contains("meteliSpotsData"))
            .last()
            .unwrap();

        let json = matching_line.split("=").last().unwrap().trim();
        let rs = serde_json::from_str::<MeteliNetPageJson>(json).unwrap();

        let mut events = Events::new();
        for gig in rs {
            events.extend(vec![Gig {
                location: gig.location,
                name: gig.name.split(":").next().unwrap().to_string(),
                website: gig.location_url.to_string(),
                age_limit: None,
                date: gig.date,
            }]);
        }

        Ok(events)
    }

    fn name(&self) -> String {
        return String::from("meteli.net");
    }
}
