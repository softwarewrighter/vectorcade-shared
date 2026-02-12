use glam::Vec2;
use vectorcade_math::{Aabb, Circle, line_aabb_intersect, line_circle_intersect};

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
    assert!(line_aabb_intersect(
        Vec2::new(-2.0, 0.0),
        Vec2::new(2.0, 0.0),
        &aabb
    ));
    assert!(!line_aabb_intersect(
        Vec2::new(-2.0, 2.0),
        Vec2::new(2.0, 2.0),
        &aabb
    ));
}

#[test]
fn line_circle() {
    let circle = Circle::new(Vec2::ZERO, 1.0);
    assert!(line_circle_intersect(
        Vec2::new(-2.0, 0.0),
        Vec2::new(2.0, 0.0),
        &circle
    ));
    assert!(!line_circle_intersect(
        Vec2::new(-2.0, 2.0),
        Vec2::new(2.0, 2.0),
        &circle
    ));
}
