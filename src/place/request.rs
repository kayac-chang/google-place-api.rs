use clap::ArgEnum;
use strum_macros::Display;

#[derive(Clone, ArgEnum, Display, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum InputType {
    TextQuery,
    PhoneNumber,
}

#[derive(Debug)]
pub struct Request<'a> {
    pub url: &'a str,
    pub token: &'a str,
    pub input: String,
    pub input_type: String,
    pub fields: Vec<Field>,
}

#[derive(Clone, ArgEnum, Display, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Field {
    // AddressComponent,
    // AdrAddress,
    // FormattedPhoneNumber,
    // InternationalPhoneNumber,
    // Review,
    // Type,
    // Url,
    // UtcOffset,
    // Website,
    // Vicinity,
    BusinessStatus,
    FormattedAddress,
    Geometry,
    Icon,
    IconMaskBaseUri,
    IconBackgroundColor,
    Name,
    Photo,
    PlaceId,
    PlusCode,

    OpeningHours,

    PriceLevel,
    Rating,
    UserRatingsTotal,
}

impl Request<'_> {
    pub fn add_field(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);

        self
    }

    pub fn add_fields(&mut self, fields: Vec<Field>) -> &mut Self {
        fields.into_iter().for_each(|field| {
            self.add_field(field);
        });

        self
    }
}
