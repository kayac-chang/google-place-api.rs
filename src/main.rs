use find_place::place::Client;
use std::env;

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let token = args.get(0).expect("should passing first argument token");

    let output = Client::new(token)
        .find("Museum of Contemporary Art Australia", "textquery")
        .add_fields(vec![
            // place::Field::BusinessStatus,
            // place::Field::FormattedAddress,
            // place::Field::Geometry,
            // place::Field::Icon,
            // place::Field::IconMaskBaseUri,
            // place::Field::IconBackgroundColor,
            // place::Field::Name,
            // place::Field::Photo,
            // place::Field::PlaceId,
            // place::Field::PlusCode,
            // place::Field::OpeningHours,
            // place::Field::PriceLevel,
            // place::Field::Rating,
            // place::Field::UserRatingsTotal,
        ])
        .send()
        .await?;

    println!("{:#?}", output);

    Ok(())
}
