use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub candidates: Vec<Place>,
    pub status: SearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
}
