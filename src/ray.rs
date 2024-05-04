use crate::position::Position;

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Position,
    direction: Position,
}

impl Ray {
    pub fn new(origin: Position, direction: Position) -> Self {
        assert!(
            (direction.length() - 1.0).abs() <= 0.0001,
            "Direction needs to be a unit vector"
        );
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Position {
        self.origin + t * self.direction
    }

    pub fn origin(self) -> Position {
        self.origin
    }

    pub fn direction(self) -> Position {
        self.direction
    }
}
