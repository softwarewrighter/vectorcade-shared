//! Projectile systems for 2D and 3D shooter games.
//!
//! Provides shared projectile logic for games like Battlezone (3D) and
//! Asteroids (2D). Rendering is game-specific; this module handles
//! movement and collision detection.

use glam::{Vec2, Vec3};

/// 3D projectile for first-person shooter games (Battlezone, etc.).
#[derive(Clone, Debug, PartialEq)]
pub struct Projectile3D {
    /// Current position in world space.
    pub pos: Vec3,
    /// Normalized direction of travel.
    pub dir: Vec3,
    /// Speed in units per second.
    pub speed: f32,
    /// Maximum travel distance before despawn.
    pub max_dist: f32,
    /// Distance traveled so far.
    pub traveled: f32,
    /// Whether the projectile is still active.
    pub alive: bool,
}

impl Projectile3D {
    /// Create a new projectile.
    ///
    /// # Arguments
    ///
    /// * `origin` - Starting position
    /// * `direction` - Direction of travel (will be normalized)
    /// * `speed` - Travel speed in units per second
    /// * `max_dist` - Maximum distance before auto-despawn
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3, speed: f32, max_dist: f32) -> Self {
        Self {
            pos: origin,
            dir: direction.normalize_or_zero(),
            speed,
            max_dist,
            traveled: 0.0,
            alive: true,
        }
    }

    /// Update projectile position. Marks as dead if max distance exceeded.
    pub fn update(&mut self, dt: f32) {
        if !self.alive {
            return;
        }
        let distance = self.speed * dt;
        self.pos += self.dir * distance;
        self.traveled += distance;
        if self.traveled >= self.max_dist {
            self.alive = false;
        }
    }

    /// Check collision against a spherical hitbox.
    #[must_use]
    pub fn hits_sphere(&self, center: Vec3, radius: f32) -> bool {
        self.alive && self.pos.distance_squared(center) <= radius * radius
    }
}

/// 2D projectile for top-down shooter games (Asteroids, etc.).
#[derive(Clone, Debug, PartialEq)]
pub struct Projectile2D {
    /// Current position in world space.
    pub pos: Vec2,
    /// Velocity (direction and speed combined).
    pub vel: Vec2,
    /// Whether the projectile is still active.
    pub alive: bool,
    /// Remaining lifetime in seconds (0 = immortal until manually killed).
    pub lifetime: f32,
}

impl Projectile2D {
    /// Create a new 2D projectile.
    ///
    /// # Arguments
    ///
    /// * `pos` - Starting position
    /// * `vel` - Velocity vector (direction * speed)
    /// * `lifetime` - Time until auto-despawn (0 = no auto-despawn)
    #[must_use]
    pub fn new(pos: Vec2, vel: Vec2, lifetime: f32) -> Self {
        Self {
            pos,
            vel,
            alive: true,
            lifetime,
        }
    }

    /// Update projectile position. Marks as dead if lifetime expired.
    pub fn update(&mut self, dt: f32) {
        if !self.alive {
            return;
        }
        self.pos += self.vel * dt;
        if self.lifetime > 0.0 {
            self.lifetime -= dt;
            if self.lifetime <= 0.0 {
                self.alive = false;
            }
        }
    }

    /// Check collision against a circular hitbox.
    #[must_use]
    pub fn hits_circle(&self, center: Vec2, radius: f32) -> bool {
        self.alive && self.pos.distance_squared(center) <= radius * radius
    }
}

/// Update all projectiles in a list and remove dead ones.
///
/// Convenience function for managing projectile lists.
pub fn update_projectiles_3d(projectiles: &mut Vec<Projectile3D>, dt: f32) {
    for p in projectiles.iter_mut() {
        p.update(dt);
    }
    projectiles.retain(|p| p.alive);
}
