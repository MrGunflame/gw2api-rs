use tokio::runtime::{Builder, Runtime};

use std::sync::Arc;

use crate::{Endpoint, Result};

#[derive(Clone, Debug)]
pub struct Client {
    inner: crate::Client,
    runtime: Arc<Runtime>,
}

impl Client {
    pub fn new() -> Self {
        let runtime = Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        Self {
            inner: crate::Client::new(),
            runtime: Arc::new(runtime),
        }
    }

    pub fn get<T>(&self) -> Result<T::Value>
    where
        T: Endpoint,
    {
        self.runtime.block_on(self.inner.get::<T>())
    }
}
