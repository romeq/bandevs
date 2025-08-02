# Bandevs-cli ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/romeq/bandevs) ![GitHub repo size](https://img.shields.io/github/repo-size/romeq/bandevs) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/romeq/bandevs/.github%2Fworkflows%2Frust.yml)



Find bands' events from terminal

*As of now, gigs are only searched from https://meteli.net for Finland. More in [Data Sources](#data-sources).*


## Features
- [x] CLI for finding upcoming gigs for bands
  - [x] Allow parsing bands from a text file
- [ ] Daemon
  - [ ] Integrate ntfy.sh


## Installation & usage
```sh
cargo install --git https://github.com/romeq/bandevs 

# Find gigs for bands from a file
echo "someband\nsome other band" > ~/.bandevs_bands
bandevs from_file --file ~/.bandevs_bands

# Find gigs for a single band
bandevs find --name "some band"
```


## Data Sources
Existing data source implementations can be found in `src/data_sources`.

### Implementing your own data source
Website integrations are built around the `DataSource` trait (defined in `data_sources/mod.rs`). 
- For an example source of https://my-data-source.com/, you should create a file called `src/data_sources/my-data-source_com.rs`.
- Example content for `my-data-source_com.rs`

  ```rs
  impl DataSource for MeteliNet {
    fn name(&self) -> String {
      return String::from("my-data-source.com")
    }
    fn get_gigs(&self, band_name: String) -> Result<Events, Error> {
      // do some magic and find results from your source
      Ok(results)
    }
  }
  ```
