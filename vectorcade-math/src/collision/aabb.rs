//! Axis-aligned bounding box collision primitive.

use glam::Vec2;

/// Axis-aligned bounding box (AABB).
///
/// Defined by minimum and maximum corner points. Used for fast
/// broadphase collision detection and spatial queries.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    /// Minimum corner (bottom-left in screen coordinates).
    pub min: Vec2,
    /// Maximum corner (top-right in screen coordinates).
    pub max: Vec2,
}

impl Aabb {
    /// Create an AABB from center point and half-extents.
    ///
    /// Half-extents define the distance from center to edge on each axis.
    #[must_use]
    pub fn from_center(center: Vec2, half: Vec2) -> Self {
        Self {
            min: center - half,
            max: center + half,
        }
    }

    /// Create an AABB from min/max corner points.
    #[must_use]
    pub const fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Return the center point of the AABB.
    #[must_use]
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Return the half-extents (half width, half height).
    #[must_use]
    pub fn half_extents(&self) -> Vec2 {
        (self.max - self.min) * 0.5
    }

    /// Check if a point is inside the AABB (inclusive bounds).
    #[must_use]
    pub fn contains_point(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Check if this AABB overlaps another AABB.
    #[must_use]
    pub fn overlaps(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}
