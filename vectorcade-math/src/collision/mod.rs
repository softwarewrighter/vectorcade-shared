//! Basic collision detection for 2D games.

mod aabb;
mod circle;

pub use aabb::Aabb;
pub use circle::Circle;

use glam::Vec2;

/// Check if a line segment intersects an AABB.
#[must_use]
pub fn line_aabb_intersect(a: Vec2, b: Vec2, aabb: &Aabb) -> bool {
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
