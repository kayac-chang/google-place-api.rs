mod request;
mod response;

use crate::fetch;
use request::Request;
use response::Response;

pub use request::Field;
pub use request::InputType;

type Error = Box<dyn std::error::Error>;

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: impl Into<String>) -> Client {
        Client {
            token: token.into(),
        }
    }

    pub fn find(&self, input: impl Into<String>, input_type: impl Into<String>) -> Request {
        Request {
            url: "https://maps.googleapis.com/maps/api/place/findplacefromtext/json",
            token: &self.token,
            input: input.into(),
            input_type: input_type.into(),
            fields: vec![],
        }
    }
}

impl Request<'_> {
    pub async fn send(&self) -> Result<Response, Error> {
        let mut params = vec![
            ("key", self.token),
            ("input", &self.input),
            ("inputtype", &self.input_type),
        ];

        let fields = self
            .fields
            .iter()
            .map(|field| field.to_string())
            .collect::<Vec<_>>()
            .join(",");

        if fields.len() > 0 {
            params.push(("fields", &fields));
        }

        let response = fetch(&self.url, &params).await?;
        Ok(response)
    }
}
