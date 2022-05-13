pub mod v2;

#[cfg(feature = "blocking")]
pub mod blocking;

use hyper::{client::connect::HttpConnector, Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use thiserror::Error;

use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Client {
    client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    pub fn new() -> Self {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        Self { client }
    }

    pub async fn get<T>(&self) -> Result<T::Value>
    where
        T: Endpoint,
    {
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("https://api.guildwars2.com{}", T::URI))
            .body(Body::empty())
            .unwrap();

        let resp = self.client.request(req).await?;

        let bytes = hyper::body::to_bytes(resp.into_body()).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get_with_id<T>(&self, id: impl Into<T::Id>) -> Result<T::SingleValue>
    where
        T: ListEndpoint,
    {
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!(
                "https://api.guildwars2.com{}?id={}",
                T::URI,
                id.into()
            ))
            .body(Body::empty())
            .unwrap();

        let resp = self.client.request(req).await?;

        let bytes = hyper::body::to_bytes(resp.into_body()).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn get_all<T>(&self) -> Result<T::MultiValue>
    where
        T: ListEndpoint,
    {
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!("https://api.guildwars2.com{}?ids=all", T::URI))
            .body(Body::empty())
            .unwrap();

        let resp = self.client.request(req).await?;

        let bytes = hyper::body::to_bytes(resp.into_body()).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] hyper::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub enum IdParameter<'a> {
    All,
    Id(u64),
    Ids(&'a [u64]),
}

impl<'a> From<u64> for IdParameter<'a> {
    fn from(id: u64) -> Self {
        Self::Id(id)
    }
}

impl<'a> From<&'a [u64]> for IdParameter<'a> {
    fn from(slice: &'a [u64]) -> Self {
        Self::Ids(slice)
    }
}

impl<'a> Display for IdParameter<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::All => write!(f, "ids=all"),
            Self::Id(id) => write!(f, "id={}", id),
            Self::Ids(ids) => {
                write!(f, "ids={}", ids[0])?;

                for id in (*ids).iter().skip(1) {
                    write!(f, ",{}", id)?;
                }

                Ok(())
            }
        }
    }
}

pub trait Endpoint {
    type Value: DeserializeOwned;

    const URI: &'static str;
    const AUTHENTICATED: bool;
    const LOCALIZED: bool;
}

pub trait ListEndpoint: Endpoint {
    /// The id type of the endpoint.
    type Id: Display;
    type SingleValue: DeserializeOwned;
    type MultiValue: DeserializeOwned;
}

#[cfg(test)]
mod tests {
    use super::IdParameter;

    #[test]
    fn test_id_parameter() {
        let param = IdParameter::All;
        assert_eq!(param.to_string(), "ids=all");

        let param = IdParameter::Id(69);
        assert_eq!(param.to_string(), "id=69");

        let param = IdParameter::Ids(&[0, 1, 2, 3]);
        assert_eq!(param.to_string(), "ids=0,1,2,3")
    }
}
