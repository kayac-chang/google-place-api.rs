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
