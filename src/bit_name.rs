use strum::{Display, EnumCount, EnumString, FromRepr};

#[repr(C)]
#[derive(Debug, Copy, Clone, EnumString, EnumCount, FromRepr, Display, PartialEq, Eq)]
pub enum BitName {
    EmojiAvoidance,
    EmojiGrabber,
    FlappyGun,
    MasterMind,
    MathQuiz,
    Puzzle15,
    ShapeGrabber,
    ShapeMasher,
    ShapeMemorizer,
    ShapeRacer,
    ShapeShooter,
    ShapeSidescroller,
    WheresWaldo,
}
