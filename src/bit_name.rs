use strum::{Display, EnumCount, EnumString, FromRepr};

#[repr(C)]
#[derive(Debug, Copy, Clone, EnumString, EnumCount, FromRepr, Display, PartialEq, Eq)]
pub enum BitName {
    EmojiAvoidance,
    EmojiCatcher,
    EmojiGrabber,
    EmojiSequencer,
    FlappyGun,
    MasterMind,
    MathQuiz,
    Memoji,
    Puzzle15,
    TowerTumble,
    WhackAMole,
    WheresWaldo,
}
