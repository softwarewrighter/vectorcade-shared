use glam::{Mat3, Vec2, Vec3};

pub fn clamp(x: f32, lo: f32, hi: f32) -> f32 { x.max(lo).min(hi) }

pub fn wrap_signed_unit(mut x: f32) -> f32 {
    // Wrap into [-1, 1)
    while x < -1.0 { x += 2.0; }
    while x >= 1.0 { x -= 2.0; }
    x
}

pub fn rot2(theta_rad: f32) -> Mat3 {
    let (s, c) = theta_rad.sin_cos();
    Mat3::from_cols_array(&[
        c,  s, 0.0,
       -s,  c, 0.0,
        0.0, 0.0, 1.0
    ])
}

pub fn translate2(t: Vec2) -> Mat3 {
    Mat3::from_cols_array(&[
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        t.x, t.y, 1.0
    ])
}

/// Minimal 3D→2D projection helper for "2.5D vector" games (Battlezone/Tempest-like).
pub fn project_persp(p: Vec3, fov_y_rad: f32, aspect: f32) -> Option<Vec2> {
    // Camera looks down -Z; clip near plane.
    let z = -p.z;
    if z <= 1e-3 { return None; }
    let f = 1.0 / (0.5 * fov_y_rad).tan();
    let x = (p.x * f / aspect) / z;
    let y = (p.y * f) / z;
    Some(Vec2::new(x, y))
}
