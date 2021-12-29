use std::fmt::Display;

use crate::{models::LatLng, SearchParams};

#[derive(Debug, Default)]
pub struct Request {
    pub token: String,

    /// The point around which to retrieve place information.
    /// This must be specified as latitude,longitude.
    pub location: LatLng,

    /// A term to be matched against all content that Google has indexed for this place,
    /// including but not limited to name and type,
    /// as well as customer reviews and other third-party content.
    /// Note that explicitly including location information using this parameter
    /// may conflict with the location, radius, and rankby parameters, causing unexpected results.
    pub keyword: Option<String>,

    /// The language in which to return results.
    /// - See the list of supported languages.
    ///   Google often updates the supported languages, so this list may not be exhaustive.
    /// - If language is not supplied,
    ///   the API attempts to use the preferred language as specified in the Accept-Language header.
    /// - The API does its best to provide a street address that is readable for both the user and locals.
    ///   To achieve that goal, it returns street addresses in the local language,
    ///   transliterated to a script readable by the user if necessary, observing the preferred language.
    ///   All other addresses are returned in the preferred language. Address components are all returned in the same language,
    ///   which is chosen from the first component.
    /// - If a name is not available in the preferred language, the API uses the closest match.
    /// - The preferred language has a small influence on the set of results that the API chooses to return,
    ///   and the order in which they are returned.
    ///   The geocoder interprets abbreviations differently depending on language,
    ///   such as the abbreviations for street types, or synonyms that may be valid in one language but not in another.
    ///   For example, utca and tér are synonyms for street in Hungarian.
    pub language: Option<String>,

    /// Restricts results to only those places within the specified range.
    /// Valid values range between 0 (most affordable) to 4 (most expensive), inclusive.
    /// The exact amount indicated by a specific value will vary from region to region.
    pub maxprice: Option<String>,

    /// Restricts results to only those places within the specified range.
    /// Valid values range between 0 (most affordable) to 4 (most expensive), inclusive.
    /// The exact amount indicated by a specific value will vary from region to region.
    pub minprice: Option<String>,

    /// Returns only those places that are open for business at the time the query is sent.
    /// Places that do not specify opening hours in the Google Places database will not be returned if you include this parameter in your query.
    pub opennow: Option<bool>,

    /// Returns up to 20 results from a previously run search.
    /// Setting a pagetoken parameter will execute a search with the same parameters used previously —
    /// all parameters other than pagetoken will be ignored.
    pub pagetoken: Option<String>,

    /// Restricts the results to places matching the specified type.
    /// Only one type may be specified.
    /// If more than one type is provided, all types following the first entry are ignored.
    pub request_type: Option<String>,
}

impl Display for LatLng {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.lat, self.lng)
    }
}

impl SearchParams for Request {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = vec![];

        params.push(("key".to_owned(), self.token.to_owned()));

        params.push(("location".to_owned(), self.location.to_string()));

        if let Some(keyword) = &self.keyword {
            params.push(("keyword".to_owned(), keyword.to_owned()))
        }

        if let Some(language) = &self.language {
            params.push(("language".to_owned(), language.to_owned()))
        }

        if let Some(maxprice) = &self.maxprice {
            params.push(("maxprice".to_owned(), maxprice.to_owned()))
        }

        if let Some(minprice) = &self.minprice {
            params.push(("minprice".to_owned(), minprice.to_owned()))
        }

        if let Some(opennow) = &self.opennow {
            if *opennow {
                params.push(("opennow".to_owned(), "true".to_owned()))
            }
        }

        if let Some(pagetoken) = &self.pagetoken {
            params.push(("pagetoken".to_owned(), pagetoken.to_owned()))
        }

        if let Some(request_type) = &self.request_type {
            params.push(("type".to_owned(), request_type.to_owned()))
        }

        params
    }
}

/// (default)
/// This option sorts results based on their importance.
/// Ranking will favor prominent places within the set radius over nearby places that match but that are less prominent.
/// Prominence can be affected by a place's ranking in Google's index, global popularity, and other factors.
/// When prominence is specified, the radius parameter is required.
pub struct Prominence {
    pub request: Request,

    /// Defines the distance (in meters) within which to return place results.
    /// You may bias results to a specified circle by passing a location and a radius parameter.
    /// Doing so instructs the Places service to prefer showing results within that circle;
    /// results outside of the defined area may still be displayed.
    ///
    /// The radius will automatically be clamped to a maximum value depending on the type of search and other parameters.
    /// - Autocomplete: 50,000 meters
    /// - Nearby Search:
    ///     - with keyword or name: 50,000 meters
    ///     - without keyword or name
    ///         - rankby=prominence (default): 50,000 meters
    ///         - rankby=distance: A few kilometers depending on density of area.
    ///                             radius will not be accepted, and will result in an INVALID_REQUEST.
    /// - Query Autocomplete: 50,000 meters
    /// - Text Search: 50,000 meters
    pub radius: u32,
}

impl SearchParams for Prominence {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = self.request.get_params();

        params.push(("rankby".to_owned(), "prominence".to_owned()));
        params.push(("radius".to_owned(), self.radius.to_string()));

        params
    }
}

/// This option biases search results in ascending order by their distance from the specified location.
/// When distance is specified, one or more of keyword, name, or type is required and radius is disallowed.
pub struct PendingDistance {
    pub request: Request,
}

impl PendingDistance {
    pub fn set_type(mut self, request_type: impl Into<String>) -> Distance {
        self.request.request_type = Some(request_type.into());

        Distance {
            request: self.request,
        }
    }

    pub fn set_keyword(mut self, keyword: impl Into<String>) -> Distance {
        self.request.keyword = Some(keyword.into());

        Distance {
            request: self.request,
        }
    }
}

pub struct Distance {
    pub request: Request,
}

impl Distance {
    pub fn set_keyword(mut self, keyword: impl Into<String>) -> Distance {
        self.request.keyword = Some(keyword.into());

        Distance {
            request: self.request,
        }
    }
}

impl SearchParams for Distance {
    fn get_params(&self) -> Vec<(String, String)> {
        let mut params = self.request.get_params();

        params.push(("rankby".to_owned(), "distance".to_owned()));

        params
    }
}

impl Request {
    pub fn prominence(self, radius: u32) -> Prominence {
        Prominence {
            request: self,
            radius,
        }
    }

    pub fn distance(self) -> PendingDistance {
        PendingDistance { request: self }
    }
}
