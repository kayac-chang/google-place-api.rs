use std::process;

use clap::{Parser, Subcommand};
use google_place_api::nearby;
use google_place_api::place;
use google_place_api::{Client, Send};

type Error = Box<dyn std::error::Error>;

#[derive(Parser)]
#[clap(about, author, version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Find(FindArgs),
    Nearby(NearbyArgs),
}

#[derive(Parser)]
struct FindArgs {
    /// identifies the search target, such as a name, address, or phone number.
    #[clap(long)]
    input: String,

    /// google api key
    #[clap(long)]
    token: String,

    /// the type of input.
    #[clap(long, arg_enum)]
    input_type: place::InputType,

    /// specify a list of place data types to return
    #[clap(long, arg_enum, multiple_values = true)]
    fields: Vec<place::Field>,
}

async fn find(config: FindArgs) -> Result<(), Error> {
    let output: place::Response = Client::new(config.token)
        .find(config.input, config.input_type.to_string())
        .add_fields(config.fields)
        .send()
        .await?;

    println!("{:#?}", output);

    Ok(())
}

#[derive(Parser)]
struct NearbyArgs {
    #[clap(subcommand)]
    command: NearbyAction,
}

#[derive(Subcommand)]
enum NearbyAction {
    Prominence(ProminenceArgs),
    Distance(DistanceArgs),
}

#[derive(Parser)]
struct ProminenceArgs {
    /// google api key
    #[clap(long)]
    token: String,

    /// The point around which to retrieve place information. This must be specified as latitude,longitude.
    #[clap(long)]
    location: String,

    /// Defines the distance (in meters) within which to return place results.
    #[clap(long)]
    radius: u32,
}

#[derive(Parser)]
struct DistanceArgs {
    /// google api key
    #[clap(long)]
    token: String,

    /// The point around which to retrieve place information. This must be specified as latitude,longitude.
    #[clap(long)]
    location: String,

    /// Restricts the results to places matching the specified type.
    #[clap(long = "type")]
    request_type: String,

    /// A term to be matched against all content that Google has indexed for this place.
    #[clap(long)]
    keyword: String,
}

async fn nearby(config: NearbyArgs) -> Result<(), Error> {
    fn parse_location(location: &str) -> (f64, f64) {
        let collect = location
            .split(",")
            .flat_map(|arg| arg.parse::<f64>())
            .collect::<Vec<_>>();

        (collect[0], collect[1])
    }

    let output: nearby::Response = match config.command {
        NearbyAction::Prominence(config) => {
            let (lat, lng) = parse_location(&config.location);

            Client::new(config.token)
                .nearby(lat, lng)
                .prominence(config.radius)
                .send()
                .await?
        }

        NearbyAction::Distance(config) => {
            let (lat, lng) = parse_location(&config.location);

            Client::new(config.token)
                .nearby(lat, lng)
                .distance()
                .set_type(config.request_type)
                .set_keyword(config.keyword)
                .send()
                .await?
        }
    };

    println!("{:#?}", output);

    Ok(())
}

#[tokio::main]
async fn main() {
    let result = match Cli::parse().command {
        Commands::Find(config) => find(config).await,
        Commands::Nearby(config) => nearby(config).await,
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);

        process::exit(1);
    }
}
