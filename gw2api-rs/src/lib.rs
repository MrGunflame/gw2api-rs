pub mod v2;

#[cfg(feature = "blocking")]
pub mod blocking;

use hyper::{client::connect::HttpConnector, Body, Method, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use thiserror::Error;

use std::fmt::{self, Display, Formatter, Write};

#[derive(Clone, Debug)]
pub struct Client {
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    access_token: Option<String>,
    language: Language,
}

impl Client {
    pub fn new() -> Self {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        Self {
            client,
            access_token: None,
            language: Language::default(),
        }
    }

    pub async fn get<T>(&self) -> Result<T::Value>
    where
        T: Endpoint,
    {
        self.request(T::URI, T::IS_AUTHENTICATED, T::IS_LOCALIZED)
            .await
    }

    pub async fn get_with_id<T, U>(&self, id: U) -> Result<T::SingleValue>
    where
        T: ListEndpoint,
        U: Into<T::Id>,
    {
        let uri = format!("{}?id={}", T::URI, id.into());

        self.request(&uri, T::IS_AUTHENTICATED, T::IS_LOCALIZED)
            .await
    }

    pub async fn get_with_ids<T, U>(&self, ids: U) -> Result<T::MultiValue>
    where
        T: ListEndpoint,
        U: AsRef<[T::Id]>,
    {
        let mut uri = format!("{}?ids={}", T::URI, ids.as_ref()[0]);

        for id in ids.as_ref().iter() {
            write!(uri, ",{}", id).unwrap();
        }

        self.request(&uri, T::IS_AUTHENTICATED, T::IS_LOCALIZED)
            .await
    }

    pub async fn get_all<T>(&self) -> Result<T::MultiValue>
    where
        T: ListEndpoint,
    {
        let uri = format!("{}?ids=all", T::URI);

        self.request(&uri, T::IS_AUTHENTICATED, T::IS_LOCALIZED)
            .await
    }

    async fn request<T>(&self, uri: &str, is_authenticated: bool, is_localized: bool) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut req = Request::builder()
            .method(Method::GET)
            .uri(format!("https://api.guildwars2.com{}", uri));

        req = req.header("X-Schema-Version", "2022-03-23T19:00:00.000Z");

        if is_authenticated {
            match &self.access_token {
                Some(access_token) => {
                    req = req.header("Authorization", format!("Bearer {}", access_token));
                }
                None => return Err(Error::NoAccessToken),
            }
        }

        if is_localized {
            req = req.uri(if uri.contains("?") {
                format!("https://api.guildwars2.com{}&lang={}", uri, self.language)
            } else {
                format!("https://api.guildwars2.com{}?lang={}", uri, self.language)
            });
        }

        println!("{:?}", req);

        let req = req.body(Body::empty()).unwrap();

        let resp = self.client.request(req).await?;

        let bytes = hyper::body::to_bytes(resp.into_body()).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Builder {
    access_token: Option<String>,
    language: Language,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn access_token(mut self, access_token: String) -> Self {
        self.access_token = Some(access_token);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }
}

impl From<Builder> for Client {
    fn from(builder: Builder) -> Self {
        let mut client = Client::new();
        client.access_token = builder.access_token;
        client.language = builder.language;
        client
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] hyper::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("no access token")]
    NoAccessToken,
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
    const IS_AUTHENTICATED: bool;
    const IS_LOCALIZED: bool;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    Es,
    De,
    Fr,
    Zh,
}

impl Default for Language {
    fn default() -> Self {
        Self::En
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            Self::En => "en",
            Self::Es => "es",
            Self::De => "de",
            Self::Fr => "fr",
            Self::Zh => "zh",
        };

        write!(f, "{}", string)
    }
}
