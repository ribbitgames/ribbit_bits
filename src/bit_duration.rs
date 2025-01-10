use core::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub struct BitDuration(Duration);

pub const MIN_SECS: Duration = Duration::from_secs(3);
pub const MAX_SECS: Duration = Duration::from_secs(120);

impl BitDuration {
    pub fn new(duration: Duration) -> Self {
        let clamped = duration.clamp(MIN_SECS, MAX_SECS);
        Self(clamped)
    }

    pub fn max_duration() -> Self {
        Self::new(MAX_SECS)
    }

    pub fn min_duration() -> Self {
        Self::new(MIN_SECS)
    }

    pub const fn get_duration(&self) -> Duration {
        self.0
    }
}

impl Default for BitDuration {
    fn default() -> Self {
        Self::new(MAX_SECS)
    }
}

impl From<Duration> for BitDuration {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

impl From<BitDuration> for Duration {
    fn from(bit_duration: BitDuration) -> Self {
        bit_duration.get_duration()
    }
}
