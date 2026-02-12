//! Circle collision primitive.

use glam::Vec2;

use super::Aabb;

/// Circle collision primitive.
///
/// Defined by center point and radius. Common for player ships,
/// bullets, and other circular game objects.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    /// Center point of the circle.
    pub center: Vec2,
    /// Radius of the circle.
    pub radius: f32,
}

impl Circle {
    /// Create a new circle with the given center and radius.
    #[must_use]
    pub const fn new(center: Vec2, radius: f32) -> Self {
        Self { center, radius }
    }

    /// Check if a point is inside the circle (inclusive boundary).
    #[must_use]
    pub fn contains_point(&self, p: Vec2) -> bool {
        self.center.distance_squared(p) <= self.radius * self.radius
    }

    /// Check if this circle overlaps another circle.
    #[must_use]
    pub fn overlaps_circle(&self, other: &Circle) -> bool {
        let r_sum = self.radius + other.radius;
        self.center.distance_squared(other.center) <= r_sum * r_sum
    }

    /// Check if this circle overlaps an AABB.
    ///
    /// Finds the closest point on the AABB to the circle center,
    /// then checks if that point is inside the circle.
    #[must_use]
    pub fn overlaps_aabb(&self, aabb: &Aabb) -> bool {
        let closest = Vec2::new(
            self.center.x.clamp(aabb.min.x, aabb.max.x),
            self.center.y.clamp(aabb.min.y, aabb.max.y),
        );
        self.contains_point(closest)
    }
}
