use bevy::prelude::*;

use crate::TIME_STEP;

#[derive(Component)]
pub struct ProjectileSpeed(pub f32);
#[derive(Component)]
pub struct DespawnBounds(pub f32, pub f32, pub f32, pub f32);
impl DespawnBounds {
    pub fn new(h: f32, v: f32) -> Self {
        Self(-h / 2.0, -v / 2.0, h / 2.0, v / 2.0)
    }
    fn outside(&self, coords: Vec3) -> bool {
        if coords.x < self.0 || coords.y < self.1 || coords.x > self.2 || coords.y > self.3 {
            true
        } else {
            false
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub speed: ProjectileSpeed,
    pub bounds: DespawnBounds,
}

fn move_projectile(mut query: Query<(&ProjectileSpeed, &mut Transform)>) {
    for (speed, mut transform) in query.iter_mut() {
        let direction = transform.rotation * Vec3::Y;
        transform.translation += direction * speed.0 * TIME_STEP;
    }
}

fn despawn_outside(mut commands: Commands, query: Query<(Entity, &DespawnBounds, &Transform)>) {
    for (entity, bounds, transform) in query.iter() {
        if bounds.outside(transform.translation) {
            commands.entity(entity).despawn();
        }
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(move_projectile)
                .with_system(despawn_outside),
        );
    }
}
