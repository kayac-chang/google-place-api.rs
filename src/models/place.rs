use crate::models::LatLng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressComponent {
    pub long_name: String,
    pub short_name: String,
    pub types: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    pub northeast: LatLng,
    pub southwest: LatLng,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub location: LatLng,
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
