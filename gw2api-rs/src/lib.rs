pub mod v2;

#[cfg(feature = "blocking")]
pub mod blocking;

use hyper::{client::connect::HttpConnector, header::AUTHORIZATION, Body, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use thiserror::Error;

use std::task::{Context, Poll};
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
    future::Future,
    pin::Pin,
};

const SCHEMA_VERSION: &'static str = "2022-03-23T19:00:00.000Z";

/// The Client for making requests.
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

    /// Creates a new [`Builder`] for a client.
    #[inline]
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

pub trait ClientExecutor<T>: private::Sealed
where
    T: DeserializeOwned,
{
    type Result;

    fn send(&self, request: RequestBuilder) -> Self::Result;
}

pub(crate) mod private {
    pub trait Sealed {}
}

impl From<Builder> for Client {
    fn from(builder: Builder) -> Self {
        let mut client = Client::new();
        client.access_token = builder.access_token;
        client.language = builder.language;
        client
    }
}

/// An alias for `Result<T, Error>`.
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

/// A builder for creating endpoint requests.
pub struct RequestBuilder {
    uri: Cow<'static, str>,
    authentication: Authentication,
    localized: bool,
}

impl RequestBuilder {
    pub(crate) fn new<T>(uri: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self {
            uri: uri.into(),
            authentication: Authentication::None,
            localized: false,
        }
    }

    pub(crate) fn authenticated(mut self, v: Authentication) -> Self {
        self.authentication = v;
        self
    }

    pub(crate) fn localized(mut self, v: bool) -> Self {
        self.localized = v;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Authentication {
    None,
    Optional,
    Required,
}

impl Authentication {
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    #[inline]
    pub fn is_optional(&self) -> bool {
        matches!(self, Self::Optional)
    }

    #[inline]
    pub fn is_required(&self) -> bool {
        matches!(self, Self::Required)
    }
}

/// All possible api languages. The default language is `En`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    Es,
    De,
    Fr,
    Zh,
}

impl Default for Language {
    #[inline]
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

/// A wrapper around a future returned by the async client.
#[must_use = "futures do nothing unless polled"]
pub struct ResponseFuture<T>(Box<dyn Future<Output = T> + Send + Sync + 'static>);

impl<T> Future for ResponseFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // UNSAFETY: We can project the pin since self is pinned.
        let fut = unsafe { self.map_unchecked_mut(|this| &mut *(this.0)) };
        fut.poll(cx)
    }
}

impl<T> ClientExecutor<T> for Client
where
    T: DeserializeOwned,
{
    type Result = ResponseFuture<Result<T>>;

    fn send(&self, builder: RequestBuilder) -> Self::Result {
        let mut req = Request::builder().uri(format!("https://api.guildwars2.com{}", builder.uri));
        req = req.header("X-Schema-Version", SCHEMA_VERSION);

        if !builder.authentication.is_none() {
            let access_token = match &self.access_token {
                Some(access_token) => access_token,
                None => return ResponseFuture(Box::new(async move { Err(Error::NoAccessToken) })),
            };

            req = req.header(AUTHORIZATION, format!("Bearer {}", access_token));
        }
        let req = req.body(Body::empty()).unwrap();

        let fut = self.client.request(req);
        ResponseFuture(Box::new(async move {
            let resp = fut.await?;
            let bytes = hyper::body::to_bytes(resp).await?;

            Ok(serde_json::from_slice(&bytes)?)
        }))
    }
}

#[doc(hidden)]
impl private::Sealed for Client {}

macro_rules! endpoint {
    // Basic endpoint (single path, no ids)
    ($target:ty, $path:expr ) => {
        impl $target {
            pub fn get<C>(client: &C) -> C::Result
            where
                C: crate::ClientExecutor<Self>,
            {
                let builder = crate::RequestBuilder::new($path);
                client.send(builder)
            }
        }
    };
    ($target:ty, $path:expr, $id:ty $(,$get_all:tt)?) => {
        impl $target {
            /// Returns the item with the given `id`.
            pub fn get<C>(client: &C, id: $id) -> C::Result
            where
                C: crate::ClientExecutor<Self>,
            {
                let uri = format!("{}?id={}", $path, id);
                client.send(crate::RequestBuilder::new(uri))
            }

            $(

            /// Returns all items.
            pub fn get_all<C>(client: &C) -> C::Result
            where
                C: crate::ClientExecutor<Vec<Self>>,
            {
                stringify!($get_all);

                let uri = format!("{}?ids=all", $path);
                client.send(crate::RequestBuilder::new(uri))
            }

            )?

            /// Returns a list of all item ids.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// # use gw2api_rs::{Client, Result};
            /// #
            /// # async fn async_main() -> Result<()> {
            /// let client = Client::new();
            #[doc = concat!("let ids: Vec<", stringify!($id) , "> = ", stringify!($target), "::ids(&client).await?;")]
            /// println!("{:?}", ids);
            /// #
            /// # Ok(())
            /// # }
            /// ```
            ///
            /// Using the [`blocking`] client:
            /// ```ignore
            /// # use gw2api_rs::Result;
            /// # use gw2api_rs::blocking::Client;
            /// #
            /// # fn main() -> Result<()> {
            /// let client = Client::new();
            #[doc = concat!("let ids: Vec<", stringify!($id), "> = ", stringify!($target), "::ids(&client)?;")]
            /// println!("{:?}", ids);
            /// #
            /// # Ok(())
            /// # }
            /// ```
            pub fn ids<C>(client: &C) -> C::Result
            where
                C: crate::ClientExecutor<Vec<$id>>,
            {
                client.send(crate::RequestBuilder::new($path))
            }
        }
    };
}

pub(crate) use endpoint;
