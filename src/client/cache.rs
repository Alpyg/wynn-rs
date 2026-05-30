use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use moka::future::Cache;
use tokio::time::{Duration, Instant};

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bucket {
    Player,
    Guild,
}

pub(super) struct BucketLimiter {
    rpm: u32,
    count: AtomicU32,
    window_start: tokio::sync::Mutex<Instant>,
}

impl BucketLimiter {
    pub fn new(rpm: u32) -> Self {
        Self {
            rpm,
            count: AtomicU32::new(0),
            window_start: tokio::sync::Mutex::new(Instant::now()),
        }
    }

    pub async fn acquire(&self) -> Result<(), WynncraftError> {
        let mut start = self.window_start.lock().await;
        let now = Instant::now();

        if now.duration_since(*start) >= Duration::from_secs(60) {
            self.count.store(0, Ordering::Relaxed);
            *start = now;
        }

        let current = self.count.fetch_add(1, Ordering::Relaxed);
        if current >= self.rpm {
            Err(WynncraftError::RateLimited)
        } else {
            Ok(())
        }
    }
}

pub(super) type ResponseCache = Cache<String, Arc<serde_json::Value>>;

pub(super) fn make_cache(ttl_secs: u64) -> ResponseCache {
    Cache::builder()
        .time_to_live(Duration::from_secs(ttl_secs))
        .build()
}
