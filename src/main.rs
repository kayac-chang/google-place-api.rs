use clap::Parser;
use google_place_api::place::{Client, Field, InputType};

type Error = Box<dyn std::error::Error>;

#[derive(Parser, Debug)]
#[clap(about, author, version)]
struct Args {
    /// identifies the search target, such as a name, address, or phone number.
    #[clap(long)]
    input: String,

    /// google api key
    #[clap(long)]
    token: String,

    /// the type of input.
    #[clap(long, arg_enum)]
    input_type: InputType,

    /// specify a list of place data types to return
    #[clap(long, arg_enum, multiple_values = true)]
    fields: Vec<Field>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Args::parse();

    let output = Client::new(config.token)
        .find(config.input, config.input_type.to_string())
        .add_fields(config.fields)
        .send()
        .await?;

    println!("{:#?}", output);

    Ok(())
}
