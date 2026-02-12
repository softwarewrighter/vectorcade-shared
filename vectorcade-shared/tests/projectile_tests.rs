use glam::{Vec2, Vec3};
use vectorcade_shared::projectile::{Projectile2D, Projectile3D, update_projectiles_3d};

#[test]
fn projectile_3d_moves() {
    let mut p = Projectile3D::new(Vec3::ZERO, Vec3::Z, 10.0, 100.0);
    p.update(1.0);
    assert!((p.pos.z - 10.0).abs() < 0.001);
    assert!((p.traveled - 10.0).abs() < 0.001);
    assert!(p.alive);
}

#[test]
fn projectile_3d_despawns_at_max_dist() {
    let mut p = Projectile3D::new(Vec3::ZERO, Vec3::X, 100.0, 50.0);
    p.update(1.0); // travels 100 units, exceeds max_dist of 50
    assert!(!p.alive);
}

#[test]
fn projectile_3d_hits_sphere() {
    let p = Projectile3D::new(Vec3::new(1.0, 0.0, 0.0), Vec3::X, 10.0, 100.0);
    assert!(p.hits_sphere(Vec3::new(1.5, 0.0, 0.0), 1.0));
    assert!(!p.hits_sphere(Vec3::new(10.0, 0.0, 0.0), 1.0));
}

#[test]
fn projectile_2d_moves() {
    let mut p = Projectile2D::new(Vec2::ZERO, Vec2::new(5.0, 0.0), 0.0);
    p.update(2.0);
    assert!((p.pos.x - 10.0).abs() < 0.001);
    assert!(p.alive); // lifetime=0 means no auto-despawn
}

#[test]
fn projectile_2d_despawns_on_lifetime() {
    let mut p = Projectile2D::new(Vec2::ZERO, Vec2::X, 1.0);
    p.update(0.5);
    assert!(p.alive);
    p.update(0.6);
    assert!(!p.alive);
}

#[test]
fn projectile_2d_hits_circle() {
    let p = Projectile2D::new(Vec2::new(1.0, 0.0), Vec2::X, 5.0);
    assert!(p.hits_circle(Vec2::new(1.5, 0.0), 1.0));
    assert!(!p.hits_circle(Vec2::new(10.0, 0.0), 1.0));
}

#[test]
fn update_projectiles_removes_dead() {
    let mut projectiles = vec![
        Projectile3D::new(Vec3::ZERO, Vec3::X, 100.0, 10.0), // will die
        Projectile3D::new(Vec3::ZERO, Vec3::Y, 10.0, 100.0), // will live
    ];
    update_projectiles_3d(&mut projectiles, 1.0);
    assert_eq!(projectiles.len(), 1);
    assert!(projectiles[0].alive);
}
