use bevy::prelude::*;

use crate::{ProjectileBundle, ProjectileSpeed, DespawnBounds};

pub struct Weapon {
    pub trigger: KeyCode,
    pub offset: Vec3,
    pub cooldown: Timer,
}

#[derive(Component)]
pub struct Weapons(pub Vec<Weapon>);

fn tick_timers(time: Res<Time>, mut query: Query<&mut Weapons>) {
    for mut weapons in query.iter_mut() {
        for weapon in weapons.0.iter_mut() {
            if !weapon.cooldown.finished() {
                weapon.cooldown.tick(time.delta());
            }
        }
    }
}

fn fire_weapons(mut commands: Commands, asset_server: Res<AssetServer>, keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Weapons, &Transform)>) {
    for (mut weapons, transform) in query.iter_mut() {
        for weapon in weapons.0.iter_mut() {
            if keyboard_input.pressed(weapon.trigger) && weapon.cooldown.finished() {
                weapon.cooldown.reset();
                println!("offset: {:?}, position change: {:?}", weapon.offset, transform.rotation * Vec3::Y * weapon.offset);
                commands.spawn((
                    ProjectileBundle {
                        speed: ProjectileSpeed(1000.0),
                        bounds: DespawnBounds::new(1600.0, 900.0)
                    },
                    SpriteBundle {
                        texture: asset_server.load("laser.png"),
                        transform: Transform {
                            translation: transform.translation + transform.rotation. * weapon.offset,
                            rotation: transform.rotation,
                            scale: Vec3::splat(0.05)
                        },
                        ..default()
                    }
                ));
            }
        }
    }
}

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(fire_weapons)
                .with_system(tick_timers)
        );
    }
}
