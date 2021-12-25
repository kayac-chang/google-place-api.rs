use reqwest;
use reqwest::Url;
use serde::de::DeserializeOwned;

type Error = Box<dyn std::error::Error>;

pub async fn fetch<T>(url: &str, params: &[(impl AsRef<str>, impl AsRef<str>)]) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let url = Url::parse_with_params(url, params)?;

    let res = reqwest::get(url).await?;

    let output = res.json().await?;

    Ok(output)
}
