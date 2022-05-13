use gw2api_rs::{blocking, Builder};

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::sync::Once;

mod v2;

pub static CLIENT: Client = Client::dangling();

pub struct Client {
    once: Once,
    inner: UnsafeCell<MaybeUninit<blocking::Client>>,
}

impl Client {
    pub const fn dangling() -> Self {
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
            let inner = unsafe { &mut *self.inner.get() };

            let mut builder = Builder::new();

            if let Ok(access_token) = std::env::var("APIKEY") {
                builder = builder.access_token(access_token);
            }

            inner.write(builder.into());
        });

        unsafe {
            let ptr = &*self.inner.get();

            ptr.assume_init_ref()
        }
    }
}

unsafe impl Sync for Client {}
