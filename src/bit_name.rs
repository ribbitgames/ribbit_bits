use strum::{Display, EnumCount, EnumString, FromRepr};

#[repr(C)]
#[derive(Debug, Copy, Clone, EnumString, EnumCount, FromRepr, Display, PartialEq, Eq)]
pub enum BitName {
    FlappyGun,
    MasterMind,
    MathQuiz,
    //Maze,
    Puzzle15,
    ShapeFinder,
    ShapeGrabber,
    ShapeMasher,
    ShapeMemorizer,
    ShapeRacer,
    ShapeSidescroller,
    ShapeShooter,
    WheresWaldo,
}
