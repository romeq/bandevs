mod data_sources;
mod utils;

use clap::{arg, command, Command};

use crate::{data_sources::EventsTrait, utils::colorize};

fn find_gigs(band: String, verbose: bool, multiple_bands_mode: bool) {
    let mut band_gigs = data_sources::Events::new();

    let gigs = data_sources::search(band.replace(" ", "+").to_string(), verbose);
    band_gigs.add_gigs(gigs);

    let gigs_count = band_gigs.len();
    if multiple_bands_mode {
        if gigs_count > 0 {
            println!(
                "{}",
                colorize(&format!("Gigs for {}:", band).to_string(), "green")
            );
        } else {
            println!("Didn't find any gigs for {}", colorize(&band, "green"))
        }
    }
    for gig in band_gigs {
        println!(
            "{} @ {}     \t{} ({})",
            gig.date,
            gig.location.split(", ").last().unwrap(),
            colorize(gig.name.split(":").next().unwrap(), "green"),
            gig.website.as_str()
        );
    }
    if multiple_bands_mode && gigs_count > 0 {
        println!("");
    }
}

fn main() {
    let args = command!()
        .arg(arg!(-v --verbose "Verbose?"))
        .subcommand(
            Command::new("from_file")
                .about("Parse bands from file")
                .arg(arg!(-f --file <FILE>).required(true)),
        )
        .subcommand(
            Command::new("find")
                .about("Find gigs for a single band")
                .arg(arg!(-n --name <NAME>).required(true)),
        )
        .subcommand_required(true)
        .get_matches();

    let verbose = args.get_one::<bool>("verbose") == Some(&true);

    if let Some(matches) = args.subcommand_matches("from_file") {
        if let Some(bands_file) = matches.get_one::<String>("file") {
            let bands_str = std::fs::read_to_string(&bands_file).unwrap();
            let bands_count = bands_str.lines().count();
            for band in bands_str.lines() {
                find_gigs(band.to_string(), verbose, bands_count > 1);
            }
        }
    }
    if let Some(matches) = args.subcommand_matches("find") {
        if let Some(band) = matches.get_one::<String>("name") {
            find_gigs(band.to_string(), verbose, false);
        }
    }
}
