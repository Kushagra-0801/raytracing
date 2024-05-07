use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

use paste::paste;

#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Position {
    e: [f64; 3],
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("x", self.x_ref())
            .field("y", self.y_ref())
            .field("z", self.z_ref())
            .finish()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{x} {y} {z}", x = self.x(), y = self.y(), z = self.z())
    }
}

macro_rules! xyz {
    ($($f: ident => $idx: literal)+) => {
        paste! {
            $(
                pub fn $f(self) -> f64 {
                    self.e[$idx]
                }
                pub fn [<$f _ref>] (&self) -> &f64 {
                    &self.e[$idx]
                }
                pub fn [<$f _mut>] (&mut self) -> &mut f64 {
                    &mut self.e[$idx]
                }
            )+
        }
    };
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    xyz! {
        x => 0
        y => 1
        z => 2
    }

    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y() * rhs.z() - rhs.y() * self.z(),
            self.z() * rhs.x() - rhs.z() * self.x(),
            self.x() * rhs.y() - rhs.x() * self.y(),
        )
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }
}

impl std::ops::Neg for Position {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::Index<usize> for Position {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => self.x_ref(),
            1 => self.y_ref(),
            2 => self.z_ref(),
            _ => unreachable!("Only x, y, z are possible"),
        }
    }
}

impl std::ops::IndexMut<usize> for Position {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => self.x_mut(),
            1 => self.y_mut(),
            2 => self.z_mut(),
            _ => unreachable!("Only x, y, z are possible"),
        }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Self::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, v: Self) {
        *self = self.clone() + v
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Self::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl std::ops::SubAssign for Position {
    fn sub_assign(&mut self, v: Self) {
        *self = self.clone() - v
    }
}

impl std::ops::Mul<f64> for Position {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Self::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl std::ops::MulAssign<f64> for Position {
    fn mul_assign(&mut self, t: f64) {
        *self = self.clone() * (t)
    }
}

impl std::ops::Mul<Position> for f64 {
    type Output = Position;

    fn mul(self, v: Position) -> Self::Output {
        v * self
    }
}

impl std::ops::Div<f64> for Position {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        self * (1.0 / t)
    }
}

impl std::ops::DivAssign<f64> for Position {
    fn div_assign(&mut self, t: f64) {
        *self = self.clone() * (1.0 / t)
    }
}

impl Sum for Position {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, p| acc + p).unwrap_or_default()
    }
}
