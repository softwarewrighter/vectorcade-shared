use glam::Vec3;
use std::f32::consts::{FRAC_PI_2, PI};
use vectorcade_math::{project_line_3d, project_persp, rotate_point_y};

const FOV: f32 = FRAC_PI_2; // 90 degrees
const ASPECT: f32 = 1.0;

#[test]
fn project_point_in_front() {
    // Point directly in front of camera
    let p = Vec3::new(0.0, 0.0, -10.0);
    let result = project_persp(p, FOV, ASPECT);
    assert!(result.is_some());
    let projected = result.unwrap();
    assert!((projected.x).abs() < 0.01);
    assert!((projected.y).abs() < 0.01);
}

#[test]
fn project_point_behind() {
    // Point behind camera
    let p = Vec3::new(0.0, 0.0, 10.0);
    let result = project_persp(p, FOV, ASPECT);
    assert!(result.is_none());
}

#[test]
fn project_line_both_in_front() {
    let a = Vec3::new(-1.0, 0.0, -5.0);
    let b = Vec3::new(1.0, 0.0, -5.0);
    let result = project_line_3d(a, b, FOV, ASPECT);
    assert!(result.is_some());
}

#[test]
fn project_line_both_behind() {
    let a = Vec3::new(-1.0, 0.0, 5.0);
    let b = Vec3::new(1.0, 0.0, 5.0);
    let result = project_line_3d(a, b, FOV, ASPECT);
    assert!(result.is_none());
}

#[test]
fn project_line_one_behind_clips() {
    // Line from in front to behind camera
    let a = Vec3::new(0.0, 0.0, -5.0); // in front
    let b = Vec3::new(0.0, 0.0, 5.0); // behind
    let result = project_line_3d(a, b, FOV, ASPECT);
    assert!(result.is_some());
    // Should return valid clipped line
}

#[test]
fn rotate_point_y_90_degrees() {
    let p = Vec3::new(1.0, 0.0, 0.0);
    let rotated = rotate_point_y(p, FRAC_PI_2);
    // After 90 degree rotation around Y, (1,0,0) -> (0,0,-1)
    assert!((rotated.x).abs() < 0.001);
    assert!((rotated.y).abs() < 0.001);
    assert!((rotated.z + 1.0).abs() < 0.001);
}

#[test]
fn rotate_point_y_180_degrees() {
    let p = Vec3::new(1.0, 2.0, 0.0);
    let rotated = rotate_point_y(p, PI);
    // After 180 degree rotation, (1,2,0) -> (-1,2,0)
    assert!((rotated.x + 1.0).abs() < 0.001);
    assert!((rotated.y - 2.0).abs() < 0.001);
    assert!((rotated.z).abs() < 0.001);
}
