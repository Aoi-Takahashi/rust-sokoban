use std::fmt::{self, Display};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
#[derive(PartialEq)]
pub enum BoxColor {
    Red,
    Blue,
}

impl Display for BoxColor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColor::Red => "red",
            BoxColor::Blue => "blue",
        })?;
        Ok(())
    }
}

pub struct Renderable {
    pub path: String,
}

pub struct Wall {}

pub struct Player {}

pub struct Box {
    pub color: BoxColor,
}

pub struct BoxSpot {
    pub color: BoxColor,
}

pub struct Movable;

pub struct Immovable;
