#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}

impl Interval {
    pub fn contains(&self, v: f64) -> bool {
        self.start <= v && v <= self.end
    }

    pub fn surrounds(&self, v: f64) -> bool {
        self.start < v && v < self.end
    }

    pub fn clamp(&self, v: f64) -> f64 {
        self.start.max(v).min(self.end)
    }
}
