mod fetch;
mod models;

pub mod nearby;
pub mod place;
pub use fetch::fetch;

use async_trait::async_trait;
use models::{Error, LatLng};
use serde::de::DeserializeOwned;

pub trait SendUrl {
    fn get_url(&self) -> &'static str;
}

pub trait SearchParams {
    fn get_params(&self) -> Vec<(String, String)>;
}

#[async_trait]
pub trait Send<Response, Error> {
    async fn send(&self) -> Result<Response, Error>;
}

#[async_trait]
impl<T, R> Send<R, Error> for T
where
    T: SearchParams + SendUrl + std::marker::Sync,
    R: DeserializeOwned,
{
    async fn send(&self) -> Result<R, Error> {
        Ok(fetch(&self.get_url(), &self.get_params()).await?)
    }
}

pub struct Client {
    token: String,
}

impl Client {
    pub fn new(token: impl Into<String>) -> Client {
        Client {
            token: token.into(),
        }
    }

    pub fn find(&self, input: impl Into<String>, input_type: impl Into<String>) -> place::Request {
        place::Request {
            url: "https://maps.googleapis.com/maps/api/place/findplacefromtext/json",
            token: self.token.clone(),
            input: input.into(),
            input_type: input_type.into(),
            ..Default::default()
        }
    }

    pub fn nearby(&self, latitude: f64, longitude: f64) -> nearby::Request {
        nearby::Request {
            url: "https://maps.googleapis.com/maps/api/place/nearbysearch/json",
            token: self.token.clone(),
            location: LatLng {
                lat: latitude,
                lng: longitude,
            },
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::{Client, SearchParams};
    use crate::models::LatLng;

    #[test]
    fn test_nearby_request() {
        let request = Client::new("hello kirby").nearby(0.0, 0.0);

        assert_eq!(request.token, "hello kirby");
        assert_eq!(request.location, LatLng { lat: 0.0, lng: 0.0 });
        assert_eq!(request.keyword, None);
        assert_eq!(request.language, None);
        assert_eq!(request.maxprice, None);
        assert_eq!(request.minprice, None);
        assert_eq!(request.opennow, None);
        assert_eq!(request.pagetoken, None);
        assert_eq!(request.request_type, None);
    }

    fn format_search_params<T>(params: &[(T, T)]) -> String
    where
        T: Deref,
        T: ToString,
    {
        params
            .into_iter()
            .map(|(a, b)| format!("({},{})", a.to_string(), b.to_string()))
            .collect::<Vec<_>>()
            .join(",")
    }

    #[test]
    fn test_nearby_prominence() {
        let request = Client::new("hello kirby").nearby(0.0, 0.0).prominence(1000);

        assert_eq!(request.radius, 1000);

        let left = vec![
            ("key", "hello kirby"),
            ("location", "0,0"),
            ("rankby", "prominence"),
            ("radius", "1000"),
        ];

        let right = request.get_params();

        assert_eq!(
            //
            format_search_params(&left),
            format_search_params(&right)
        )
    }

    #[test]
    fn test_nearby_distance() {
        let request = Client::new("hello kirby")
            .nearby(0.0, 0.0)
            .distance()
            .set_type("restaurant")
            .set_keyword("food");

        let left = vec![
            ("key", "hello kirby"),
            ("location", "0,0"),
            ("keyword", "food"),
            ("type", "restaurant"),
            ("rankby", "distance"),
        ];
        let right = request.get_params();

        assert_eq!(
            //
            format_search_params(&left),
            format_search_params(&right)
        )
    }
}
