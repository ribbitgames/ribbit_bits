use std::time::Duration;

use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::BitDuration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BitResult {
    HighestScore(i64),
    LowestScore(i64),
    LongestDuration(Duration),
    FastestDuration(Duration),
    Success,
    Failure,
}

#[derive(Debug, Clone, Eq, PartialEq, EnumString, Serialize, Deserialize)]
pub enum RibbitMessage {
    End,
    Parameters, // Ribbit is requesting the bit parameters
    Start,
    Restart,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitParameters {
    pub duration: BitDuration, // How long is the game. See `BitDuration` for duration limits.
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitMessage {
    End(BitResult), // Game is completed or has been requested to end, send back the result
    Parameters(BitParameters), // Parameters specific to this bit. See `BitParameters`
    Ready,          // Sent when everything is loaded and bit is ready to be played
    Start,          // Sent when bit gets focus and the bit should start playing.
}
