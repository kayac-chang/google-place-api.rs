use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub html_attributions: Vec<String>,
    pub results: Vec<Place>,
    pub status: SearchStatus,
    pub error_message: Option<String>,
    pub info_messages: Option<Vec<String>>,
    pub next_page_token: Option<String>,
}
