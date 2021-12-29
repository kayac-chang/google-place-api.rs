mod place;
mod search_status;

pub use place::*;
pub use search_status::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

pub type Error = Box<dyn std::error::Error>;
