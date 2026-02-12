//! Basic collision detection helpers for 2D games.

use glam::Vec2;

/// Axis-aligned bounding box.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb {
    /// Create AABB from center and half-extents.
    #[must_use]
    pub fn from_center(center: Vec2, half: Vec2) -> Self {
        Self {
            min: center - half,
            max: center + half,
        }
    }

    /// Create AABB from min/max corners.
    #[must_use]
    pub const fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Center point of the AABB.
    #[must_use]
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Half-extents (half width, half height).
    #[must_use]
    pub fn half_extents(&self) -> Vec2 {
        (self.max - self.min) * 0.5
    }

    /// Check if point is inside the AABB.
    #[must_use]
    pub fn contains_point(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Check if two AABBs overlap.
    #[must_use]
    pub fn overlaps(&self, other: &Aabb) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    /// Expand AABB by a margin on all sides.
    #[must_use]
    pub fn expand(&self, margin: f32) -> Self {
        Self {
            min: self.min - Vec2::splat(margin),
            max: self.max + Vec2::splat(margin),
        }
    }
}

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
        // Find closest point on AABB to circle center
        let closest = Vec2::new(
            self.center.x.clamp(aabb.min.x, aabb.max.x),
            self.center.y.clamp(aabb.min.y, aabb.max.y),
        );
        self.contains_point(closest)
    }
}

/// Check if a line segment intersects an AABB.
#[must_use]
pub fn line_aabb_intersect(a: Vec2, b: Vec2, aabb: &Aabb) -> bool {
    // Check if either endpoint is inside
    if aabb.contains_point(a) || aabb.contains_point(b) {
        return true;
    }

    let d = b - a;
    let mut t_min = 0.0_f32;
    let mut t_max = 1.0_f32;

    // Check X slab
    if d.x.abs() > 1e-9 {
        let t1 = (aabb.min.x - a.x) / d.x;
        let t2 = (aabb.max.x - a.x) / d.x;
        let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
        t_min = t_min.max(t_near);
        t_max = t_max.min(t_far);
    } else if a.x < aabb.min.x || a.x > aabb.max.x {
        return false;
    }

    // Check Y slab
    if d.y.abs() > 1e-9 {
        let t1 = (aabb.min.y - a.y) / d.y;
        let t2 = (aabb.max.y - a.y) / d.y;
        let (t_near, t_far) = if t1 < t2 { (t1, t2) } else { (t2, t1) };
        t_min = t_min.max(t_near);
        t_max = t_max.min(t_far);
    } else if a.y < aabb.min.y || a.y > aabb.max.y {
        return false;
    }

    t_min <= t_max
}

/// Check if a line segment intersects a circle.
#[must_use]
pub fn line_circle_intersect(a: Vec2, b: Vec2, circle: &Circle) -> bool {
    let d = b - a;
    let f = a - circle.center;

    let a_coef = d.dot(d);
    let b_coef = 2.0 * f.dot(d);
    let c_coef = f.dot(f) - circle.radius * circle.radius;

    let discriminant = b_coef * b_coef - 4.0 * a_coef * c_coef;
    if discriminant < 0.0 {
        return false;
    }

    let sqrt_disc = discriminant.sqrt();
    let t1 = (-b_coef - sqrt_disc) / (2.0 * a_coef);
    let t2 = (-b_coef + sqrt_disc) / (2.0 * a_coef);

    (0.0..=1.0).contains(&t1) || (0.0..=1.0).contains(&t2) || (t1 < 0.0 && t2 > 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_overlap() {
        let a = Aabb::from_center(Vec2::ZERO, Vec2::splat(1.0));
        let b = Aabb::from_center(Vec2::new(1.5, 0.0), Vec2::splat(1.0));
        assert!(a.overlaps(&b));

        let c = Aabb::from_center(Vec2::new(3.0, 0.0), Vec2::splat(1.0));
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn circle_overlap() {
        let a = Circle::new(Vec2::ZERO, 1.0);
        let b = Circle::new(Vec2::new(1.5, 0.0), 1.0);
        assert!(a.overlaps_circle(&b));

        let c = Circle::new(Vec2::new(3.0, 0.0), 1.0);
        assert!(!a.overlaps_circle(&c));
    }

    #[test]
    fn circle_aabb_overlap() {
        let circle = Circle::new(Vec2::new(2.0, 0.0), 0.5);
        let aabb = Aabb::from_center(Vec2::ZERO, Vec2::splat(2.0));
        assert!(circle.overlaps_aabb(&aabb));
    }

    #[test]
    fn line_aabb() {
        let aabb = Aabb::from_center(Vec2::ZERO, Vec2::splat(1.0));
        assert!(line_aabb_intersect(Vec2::new(-2.0, 0.0), Vec2::new(2.0, 0.0), &aabb));
        assert!(!line_aabb_intersect(Vec2::new(-2.0, 2.0), Vec2::new(2.0, 2.0), &aabb));
    }

    #[test]
    fn line_circle() {
        let circle = Circle::new(Vec2::ZERO, 1.0);
        assert!(line_circle_intersect(Vec2::new(-2.0, 0.0), Vec2::new(2.0, 0.0), &circle));
        assert!(!line_circle_intersect(Vec2::new(-2.0, 2.0), Vec2::new(2.0, 2.0), &circle));
    }
}
