use crate::SearchParams;
use clap::ArgEnum;
use strum_macros::Display;

#[derive(Clone, ArgEnum, Display, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum InputType {
    TextQuery,
    PhoneNumber,
}

#[derive(Debug, Default)]
pub struct Request {
    pub token: String,
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

impl Request {
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

impl SearchParams for Request {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = vec![];

        params.push(("key".to_owned(), self.token.to_owned()));

        params.push(("input".to_owned(), self.input.clone()));

        params.push(("inputtype".to_owned(), self.input_type.clone()));

        if self.fields.len() > 0 {
            let fields = self
                .fields
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<_>>()
                .join(",");

            params.push(("fields".to_owned(), fields));
        }

        params
    }
}
