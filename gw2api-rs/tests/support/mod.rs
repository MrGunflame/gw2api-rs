use std::{cell::UnsafeCell, mem::MaybeUninit, ops::Deref, sync::Once};

use gw2api_rs::{blocking, Builder};

pub static CLIENT: Client = Client::dangling();

pub struct Client {
    once: Once,
    inner: UnsafeCell<MaybeUninit<blocking::Client>>,
}

impl Client {
    const fn dangling() -> Self {
        Self {
            once: Once::new(),
            inner: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
}

impl Deref for Client {
    type Target = blocking::Client;

    fn deref(&self) -> &Self::Target {
        self.once.call_once(|| {
            // SAFETY: Client can only be accessed by a single thread.
            let inner = unsafe { &mut *self.inner.get() };

            let mut builder = Builder::new();
            if let Ok(access_token) = std::env::var("APIKEY") {
                builder = builder.access_token(access_token);
            }

            inner.write(builder.into());
        });

        // SAFETY: Client is properly initialized.
        unsafe { (&*self.inner.get()).assume_init_ref() }
    }
}

unsafe impl Sync for Client {}
