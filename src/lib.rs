pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const MOVE_ACCL: f32 = 0.2;
pub const TURN_ACCL: f32 = 0.2;
pub const MOVE_DRAG: f32 = 1.03;
pub const TURN_DRAG: f32 = 1.05;

mod movement;
mod projectile;
mod weapon;

pub use movement::{
    Bounds, MoveControls, MovementBundle, MovementPlugin, Thrust, TurnSpeed, Velocity,
};
pub use projectile::{DespawnBounds, ProjectileBundle, ProjectilePlugin, ProjectileSpeed};
pub use weapon::{Weapons, Weapon, WeaponsPlugin};
