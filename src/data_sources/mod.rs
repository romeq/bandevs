mod meteli_net;

use std::fmt::Error;

use crate::utils;

use self::meteli_net::MeteliNet;

pub struct Gig {
    pub location: String,
    pub age_limit: Option<u8>,
    pub date: String,
    pub website: String,
    pub name: String,
}

pub type Events = Vec<Gig>;

pub trait EventsTrait {
    fn new() -> Self;
    fn add_gigs(&mut self, gigs: Events);
}

impl EventsTrait for Events {
    fn new() -> Self {
        Vec::new()
    }

    fn add_gigs(&mut self, gigs: Events) {
        self.extend(gigs);
    }
}

pub trait DataSource {
    fn get_gigs(&self, band_name: String) -> Result<Events, Error>;
    fn name(&self) -> String;
}

pub fn search(band_name: String, verbose: bool) -> Events {
    let mut band_events = vec![];

    let sources: Vec<Box<dyn DataSource>> = vec![Box::new(MeteliNet)];

    for source in sources {
        if verbose {
            println!(
                "[+] Finding events with artist \"{}\" from {}",
                utils::colorize(band_name.as_str(), "green"),
                source.name()
            );
        }
        let source_events = source.get_gigs(band_name.clone());
        band_events.extend(source_events.unwrap());
    }

    return band_events;
}
