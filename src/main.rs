mod data_sources;
mod utils;

use clap::Parser;

use crate::{data_sources::EventsTrait, utils::colorize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    bands_file: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let bands = std::fs::read_to_string(&args.bands_file).unwrap();
    for band in bands.lines() {
        let mut all_gigs = data_sources::Events::new();

        let gigs = data_sources::search(band.replace(" ", "+").to_string(), args.verbose);
        all_gigs.add_gigs(gigs);

        for gig in all_gigs {
            println!(
                "{} @ {}     \t{} ({})",
                gig.date,
                gig.location.split(", ").last().unwrap(),
                colorize(gig.name.split(":").next().unwrap(), "green"),
                gig.website.as_str()
            );
        }
    }
}
