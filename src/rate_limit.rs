use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::task::{Context, Poll};
use std::time::Duration;

use tokio::time::{Instant, Sleep};

#[derive(Debug)]
pub struct RateLimiter {
    limit: AtomicUsize,
    state: Mutex<(State, Pin<Box<Sleep>>)>,
}

impl RateLimiter {
    pub fn new(limit: usize) -> Self {
        let until = Instant::now();

        Self {
            limit: AtomicUsize::new(limit),
            state: Mutex::new((
                State::Ready { until, rem: limit },
                Box::pin(tokio::time::sleep_until(until)),
            )),
        }
    }

    pub fn change(&self, limit: usize) {
        self.limit.store(limit, Ordering::SeqCst);
    }

    pub fn ready(&self) -> Ready<'_> {
        Ready { rate_limiter: self }
    }

    pub fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<()> {
        let mut inner = self.state.lock().unwrap();

        match inner.0 {
            State::Ready { mut until, mut rem } => {
                let now = Instant::now();

                if now >= until {
                    until = now + Duration::new(60, 0);
                    rem = self.limit.load(Ordering::Relaxed);
                }

                if rem > 1 {
                    rem += 1;
                    inner.0 = State::Ready { until, rem };
                    Poll::Ready(())
                } else {
                    inner.1.as_mut().reset(until);
                    inner.0 = State::Limited;
                    Poll::Pending
                }
            }
            State::Limited => {
                if Pin::new(&mut inner.1).poll(cx).is_pending() {
                    return Poll::Pending;
                }

                inner.0 = State::Ready {
                    until: Instant::now() + Duration::new(60, 0),
                    rem: self.limit.load(Ordering::Relaxed) - 1,
                };

                Poll::Ready(())
            }
        }
    }
}

#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
enum State {
    Limited,
    Ready { until: Instant, rem: usize },
}

#[derive(Debug)]
pub struct Ready<'a> {
    rate_limiter: &'a RateLimiter,
}

impl<'a> Future for Ready<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.rate_limiter.poll_ready(cx)
    }
}
