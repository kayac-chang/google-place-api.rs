use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchStatus {
    /// indicating the API request was successful.
    Ok,

    /// indicating that the search was successful but returned no results.
    /// This may occur if the search was passed a latlng in a remote location.
    ZeroResults,

    /// indicating the API request was malformed,
    /// generally due to missing required query parameter (location or radius).   
    InvalidRequest,

    /// indicating any of the following:
    /// - You have exceeded the QPS limits.
    /// - Billing has not been enabled on your account.
    /// - The monthly $200 credit, or a self-imposed usage cap, has been exceeded.
    /// - The provided method of payment is no longer valid (for example, a credit card has expired).
    ///
    /// See the Maps FAQ for more information about how to resolve this error.
    OverQueryLimit,

    /// indicating that your request was denied, generally because:
    /// - The request is missing an API key.
    /// - The key parameter is invalid.
    RequestDenied,

    /// indicating an unknown error.
    UnknownError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressComponent {
    pub long_name: String,
    pub short_name: String,
    pub types: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatLngLiteral {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    pub northeast: LatLngLiteral,
    pub southwest: LatLngLiteral,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub location: LatLngLiteral,
    pub viewport: Bounds,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOpeningHoursPeriodDetail {
    pub day: u8,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOpeningHoursPeriod {
    pub close: PlaceOpeningHoursPeriodDetail,
    pub open: PlaceOpeningHoursPeriodDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceOpeningHours {
    pub open_now: Option<bool>,
    pub periods: Option<Vec<PlaceOpeningHoursPeriod>>,
    pub weekday_text: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlacePhoto {
    pub height: u32,
    pub width: u32,
    pub photo_reference: String,
    pub html_attributions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlusCode {
    pub global_code: String,
    pub compound_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceReview {
    pub author_name: String,
    pub rating: u8,
    pub relative_time_description: String,
    pub time: u32,
    pub author_url: Option<String>,
    pub language: Option<String>,
    pub profile_photo_url: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    pub address_components: Option<AddressComponent>,
    pub adr_address: Option<String>,
    pub business_status: Option<String>,
    pub formatted_address: Option<String>,
    pub formatted_phone_number: Option<String>,
    pub geometry: Option<Geometry>,
    pub icon: Option<String>,
    pub icon_background_color: Option<String>,
    pub icon_mask_base_uri: Option<String>,
    pub international_phone_number: Option<String>,
    pub name: Option<String>,
    pub opening_hours: Option<PlaceOpeningHours>,
    pub photos: Option<Vec<PlacePhoto>>,
    pub place_id: Option<String>,
    pub plus_code: Option<PlusCode>,
    pub price_level: Option<u8>,
    pub rating: Option<f32>,
    pub reviews: Option<Vec<PlaceReview>>,
    pub types: Option<Vec<String>>,
    pub url: Option<String>,
    pub user_ratings_total: Option<u32>,
    pub utc_offset: Option<i16>,
    pub vicinity: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub candidates: Vec<Place>,
    pub status: SearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
}
