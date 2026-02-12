//! Circle collision primitive.

use glam::Vec2;

use super::Aabb;

/// Circle defined by center and radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    #[must_use]
    pub const fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check if point is inside the circle.
    #[must_use]
    pub fn contains_point(&self, p: Vec2) -> bool {
        self.center.distance_squared(p) <= self.radius * self.radius
    }

    /// Check if two circles overlap.
    #[must_use]
    pub fn overlaps_circle(&self, other: &Circle) -> bool {
        let r_sum = self.radius + other.radius;
        self.center.distance_squared(other.center) <= r_sum * r_sum
    }

    /// Check if circle overlaps an AABB.
    #[must_use]
    pub fn overlaps_aabb(&self, aabb: &Aabb) -> bool {
        let closest = Vec2::new(
            self.center.x.clamp(aabb.min.x, aabb.max.x),
            self.center.y.clamp(aabb.min.y, aabb.max.y),
        );
        self.contains_point(closest)
    }
}
