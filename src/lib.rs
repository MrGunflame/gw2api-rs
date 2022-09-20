//! A wrapper for the official Guild Wars 2 API
//!
//! # Usage
//!
//! ```
//! use gw2api_rs::v2::build::Build;
//! use gw2api_rs::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!     let build = Build::get(&client).await?;
//!
//!     println!("{}", build.id);
//!     Ok(())
//! }
//! ```

pub mod v2;

#[cfg(feature = "blocking")]
pub mod blocking;

use hyper::{client::connect::HttpConnector, header::AUTHORIZATION, Body, Request};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use thiserror::Error;

use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

const SCHEMA_VERSION: &str = "2022-03-23T19:00:00.000Z";

/// The Client for making requests.
#[derive(Clone, Debug)]
pub struct Client {
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    access_token: Option<String>,
    language: Language,
}

impl Client {
    /// Creates a new `Client`.
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

impl Default for Client {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Builder {
    access_token: Option<String>,
    language: Language,
}

impl Builder {
    /// Creates a new `Builder`.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn access_token<T>(mut self, access_token: T) -> Self
    where
        T: ToString,
    {
        self.access_token = Some(access_token.to_string());
        self
    }

    /// Sets the prefered [`Language`] for this `Client`.
    #[inline]
    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }
}

/// A client used to make requests to the API.
///
/// This trait is sealed and cannot be implemented.
pub trait ClientExecutor<T>: private::Sealed
where
    T: DeserializeOwned,
{
    /// The return type of this client.
    type Result;

    /// Sends a requests returning the client's return type.
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

/// An error that may occur when making API requests.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Returns `true` if this error occured while making a HTTP request.
    #[inline]
    pub fn is_http(&self) -> bool {
        matches!(self.kind, ErrorKind::Http(_))
    }

    /// Returns `true` if this error occured while deserializing json.
    #[inline]
    pub fn is_json(&self) -> bool {
        matches!(self.kind, ErrorKind::Json(_))
    }
}

impl Error {
    fn from<T>(err: T) -> Self
    where
        T: Into<ErrorKind>,
    {
        Self { kind: err.into() }
    }
}

#[derive(Debug, Error)]
enum ErrorKind {
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    Http(#[from] hyper::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("no access token")]
    NoAccessToken,
}

#[derive(Clone, Debug, Error, Deserialize)]
#[error("api error: {text}")]
struct ApiError {
    text: String,
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
    Required,
}

impl Authentication {
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
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
pub struct ResponseFuture<T>
where
    T: DeserializeOwned,
{
    state: State<T>,
    _marker: PhantomData<T>,
    is_error: bool,
}

impl<T> ResponseFuture<T>
where
    T: DeserializeOwned,
{
    fn new(fut: hyper::client::ResponseFuture) -> Self {
        Self {
            state: State::Response(fut),
            _marker: PhantomData,
            is_error: false,
        }
    }

    fn result(res: Result<T>) -> Self {
        Self {
            state: State::Result(Some(res)),
            _marker: PhantomData,
            is_error: false,
        }
    }
}

enum State<T>
where
    T: DeserializeOwned,
{
    Response(hyper::client::ResponseFuture),
    Body(Pin<Box<dyn Future<Output = hyper::Result<hyper::body::Bytes>> + Send + Sync + 'static>>),
    Result(Option<Result<T>>),
}

impl<T> Future for ResponseFuture<T>
where
    T: DeserializeOwned,
{
    type Output = Result<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut self.state {
            State::Response(_) => {
                let fut = unsafe {
                    self.as_mut()
                        .map_unchecked_mut(|this| match &mut this.state {
                            State::Response(resp) => resp,
                            _ => unreachable!(),
                        })
                };

                match fut.poll(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(Err(err)) => Poll::Ready(Err(Error::from(err))),
                    Poll::Ready(Ok(resp)) => {
                        if !resp.status().is_success() {
                            self.is_error = true;
                        }
                        let is_error = self.is_error;

                        self.state =
                            State::Body(Box::pin(async move { hyper::body::to_bytes(resp).await }));

                        let fut = unsafe {
                            self.map_unchecked_mut(|this| match &mut this.state {
                                State::Body(body) => body,
                                _ => unreachable!(),
                            })
                        };

                        match fut.poll(cx) {
                            Poll::Pending => Poll::Pending,
                            Poll::Ready(Err(err)) => Poll::Ready(Err(Error::from(err))),
                            Poll::Ready(Ok(buf)) => {
                                if is_error {
                                    return match serde_json::from_slice::<ApiError>(&buf) {
                                        Ok(st) => Poll::Ready(Err(Error::from(st))),
                                        Err(err) => Poll::Ready(Err(Error::from(err))),
                                    };
                                }

                                match serde_json::from_slice(&buf) {
                                    Ok(st) => Poll::Ready(Ok(st)),
                                    Err(err) => Poll::Ready(Err(Error::from(err))),
                                }
                            }
                        }
                    }
                }
            }
            State::Body(fut) => {
                let fut = fut.as_mut();
                match fut.poll(cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(Err(err)) => Poll::Ready(Err(Error::from(err))),
                    Poll::Ready(Ok(buf)) => {
                        if self.is_error {
                            return match serde_json::from_slice::<ApiError>(&buf) {
                                Ok(st) => Poll::Ready(Err(Error::from(st))),
                                Err(err) => Poll::Ready(Err(Error::from(err))),
                            };
                        }

                        match serde_json::from_slice(&buf) {
                            Ok(st) => Poll::Ready(Ok(st)),
                            Err(err) => Poll::Ready(Err(Error::from(err))),
                        }
                    }
                }
            }
            State::Result(res) => Poll::Ready(res.take().unwrap()),
        }
    }
}

impl<T> Unpin for ResponseFuture<T> where T: DeserializeOwned {}

impl<T> ClientExecutor<T> for Client
where
    T: DeserializeOwned,
{
    type Result = ResponseFuture<T>;

    fn send(&self, builder: RequestBuilder) -> Self::Result {
        let mut req = Request::builder().uri(format!("https://api.guildwars2.com{}", builder.uri));
        req = req.header("X-Schema-Version", SCHEMA_VERSION);

        if !builder.authentication.is_none() {
            let access_token = match &self.access_token {
                Some(access_token) => access_token,
                None => return ResponseFuture::result(Err(Error::from(ErrorKind::NoAccessToken))),
            };

            req = req.header(AUTHORIZATION, format!("Bearer {}", access_token));
        }
        let req = req.body(Body::empty()).unwrap();

        let fut = self.client.request(req);
        ResponseFuture::new(fut)
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
            ///
            /// [`blocking`]: crate::blocking
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
