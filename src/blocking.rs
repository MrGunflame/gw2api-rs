//! Drop in place sync api client.
//!
//! Requires the `blocking` feature to be enabled.
//!
//! # Usage
//!
//! ```no_run
//! use gw2api_rs::v2::build::Build;
//! use gw2api_rs::blocking::Client;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!     let build = Build::get(&client)?;
//!
//!     println!("{}", build.id);
//!     Ok(())
//! }
//! ```

use crate::{private, Builder, ClientExecutor, RequestBuilder, Result};

use serde::de::DeserializeOwned;
use tokio::runtime::{self, Runtime};

use std::sync::Arc;

/// The synchronous api client.
#[derive(Clone, Debug)]
pub struct Client {
    inner: crate::Client,
    runtime: Arc<Runtime>,
}

impl Client {
    /// Creates a new `Client`.
    pub fn new() -> Self {
        Self::new_with_inner(crate::Client::new())
    }

    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }

    fn new_with_inner(inner: crate::Client) -> Self {
        let runtime = runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        Self {
            inner,
            runtime: Arc::new(runtime),
        }
    }
}

impl Default for Client {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<Builder> for Client {
    fn from(builder: Builder) -> Self {
        let inner = builder.into();

        Self::new_with_inner(inner)
    }
}

impl<T> ClientExecutor<T> for Client
where
    T: DeserializeOwned,
{
    type Result = Result<T>;

    fn send(&self, builder: RequestBuilder) -> Self::Result {
        self.runtime.block_on(self.inner.send(builder))
    }
}

#[doc(hidden)]
impl private::Sealed for Client {}
