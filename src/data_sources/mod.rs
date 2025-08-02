mod meteli_net;

use std::fmt::Error;

use crate::utils;

use self::meteli_net::MeteliNet;

pub struct Gig {
    pub location: String,
    pub date: String,
    pub website: String,
    pub name: String,
}

pub type Events = Vec<Gig>;

// EventsTrait enhances readibility of a DataSource by creating an alias to Vec's extend method
pub trait EventsTrait {
    fn add_gigs(&mut self, gigs: Events);
}

impl EventsTrait for Events {
    fn add_gigs(&mut self, gigs: Events) {
        self.extend(gigs);
    }
}

// DataSource trait
pub trait DataSource {
    fn get_gigs(&self, band_name: String) -> Result<Events, Error>;
    fn name(&self) -> String;
}

/// Allows for searching all implemented data sources for band's gigs
pub fn search(band_name: String, verbose: bool) -> Events {
    let mut band_events = vec![];

    let sources: Vec<Box<dyn DataSource>> = vec![Box::new(MeteliNet)];

    for source in sources {
        if verbose {
            println!("[+] Finding events with artist \"{}\" from source {}", 
                utils::colorize(band_name.as_str(), "green"), utils::colorize(source.name().as_str(), "blue") );
        }
        let source_events = source.get_gigs(band_name.clone());
        band_events.extend(source_events.unwrap());
    }

    return band_events;
}
