use std::fmt::{Debug, Display};

use paste::paste;

use crate::position::Position;

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Color {
    c: [i32; 3],
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Color")
            .field("r", self.r_ref())
            .field("g", self.g_ref())
            .field("b", self.b_ref())
            .finish()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{r} {g} {b}", r = self.r(), g = self.g(), b = self.b())
    }
}

macro_rules! rgb {
    ($($f: ident => $idx: literal)+) => {
        paste! {
            $(
                pub fn $f(self) -> i32 {
                    self.c[$idx]
                }
                pub fn [<$f _ref>] (&self) -> &i32 {
                    &self.c[$idx]
                }
                #[allow(dead_code)]
                pub fn [<$f _mut>] (&mut self) -> &mut i32 {
                    &mut self.c[$idx]
                }
            )+
        }
    };
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self { c: [r, g, b] }
    }

    rgb! {
        r => 0
        g => 1
        b => 2
    }
}

impl From<Position> for Color {
    fn from(v: Position) -> Self {
        assert!(
            0.0 <= v.x() && v.x() <= 1.0,
            "Red component of color is out of range"
        );
        assert!(
            0.0 <= v.y() && v.y() <= 1.0,
            "Green component of color is out of range"
        );
        assert!(
            0.0 <= v.z() && v.z() <= 1.0,
            "Blue component of color is out of range"
        );
        Self::new(
            (v.x() * 255.999).trunc() as i32,
            (v.y() * 255.999).trunc() as i32,
            (v.z() * 255.999).trunc() as i32,
        )
    }
}

impl Into<Position> for Color {
    fn into(self) -> Position {
        Position::new(self.r().into(), self.g().into(), self.b().into())
    }
}
