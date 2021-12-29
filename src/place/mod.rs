mod request;
mod response;

pub use request::*;
pub use response::*;

use crate::fetch;
use crate::models::Error;
use crate::SearchParams;
use crate::Send;
use async_trait::async_trait;

const URL: &'static str = "https://maps.googleapis.com/maps/api/place/findplacefromtext/json";

#[async_trait]
impl Send<Response, Error> for Request {
    async fn send(&self) -> Result<Response, Error> {
        let response = fetch(URL, &self.get_params()).await?;

        Ok(response)
    }
}
