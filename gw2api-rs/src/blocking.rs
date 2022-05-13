use crate::{Builder, Endpoint, ListEndpoint, Result};

use tokio::runtime::{self, Runtime};

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Client {
    inner: crate::Client,
    runtime: Arc<Runtime>,
}

impl Client {
    pub fn new() -> Self {
        Self::new_with_inner(crate::Client::new())
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

    pub fn get<T>(&self) -> Result<T::Value>
    where
        T: Endpoint,
    {
        self.runtime.block_on(self.inner.get::<T>())
    }

    pub fn get_with_id<T, U>(&self, id: U) -> Result<T::SingleValue>
    where
        T: ListEndpoint,
        U: Into<T::Id>,
    {
        self.runtime.block_on(self.inner.get_with_id::<T, U>(id))
    }

    pub fn get_with_ids<T, U>(&self, ids: U) -> Result<T::MultiValue>
    where
        T: ListEndpoint,
        U: AsRef<[T::Id]>,
    {
        self.runtime.block_on(self.inner.get_with_ids::<T, U>(ids))
    }

    pub fn get_all<T>(&self) -> Result<T::MultiValue>
    where
        T: ListEndpoint,
    {
        self.runtime.block_on(self.inner.get_all::<T>())
    }
}

impl From<Builder> for Client {
    fn from(builder: Builder) -> Self {
        let inner = builder.into();

        Self::new_with_inner(inner)
    }
}
