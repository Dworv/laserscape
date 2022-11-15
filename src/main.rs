use bevy::prelude::*;

use laserscape::{
    Bounds, DespawnBounds, MoveControls, MovementBundle, MovementPlugin, ProjectileBundle,
    ProjectilePlugin, ProjectileSpeed, Thrust, TurnSpeed, Velocity, Weapon, Weapons, WeaponsPlugin
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MovementPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(WeaponsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    let mut dimensions = (0.0, 0.0);
    for window in windows.iter_mut() {
        window.set_title(String::from("Illabor"));
        dimensions = (window.width(), window.height());
    }

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(0.2),
                ..default()
            },
            texture: asset_server.load("ship.png"),
            ..default()
        },
        MovementBundle {
            velocity: Velocity(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),

            bounds: Bounds::new(dimensions.0, dimensions.1),
            turn_speed: TurnSpeed::default(),
            thrust: Thrust::default(),
            controls: MoveControls {
                up: KeyCode::W,
                down: KeyCode::S,
                left: KeyCode::A,
                right: KeyCode::D,
            },
        },
        Weapons (
            vec![
                Weapon {
                    trigger: KeyCode::C,
                    offset: Vec3{ x: 20.0, y: 5.0, z: 0.0 },
                    cooldown: Timer::from_seconds(0.15, TimerMode::Once)
                },
                Weapon {
                    trigger: KeyCode::C,
                    offset: Vec3{ x: -20.0, y: 5.0, z: 0.0 },
                    cooldown: Timer::from_seconds(0.15, TimerMode::Once)
                }
            ]
        )
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::splat(0.2),
                ..default()
            },
            texture: asset_server.load("ship.png"),
            ..default()
        },
        MovementBundle {
            velocity: Velocity(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),

            bounds: Bounds::new(dimensions.0, dimensions.1),
            turn_speed: TurnSpeed::default(),
            thrust: Thrust::default(),
            controls: MoveControls {
                up: KeyCode::I,
                down: KeyCode::K,
                left: KeyCode::J,
                right: KeyCode::L,
            },
        },
        Weapons (
            vec![
                Weapon {
                    trigger: KeyCode::N,
                    offset: Vec3{ x: 20.0, y: 5.0, z: 0.0 },
                    cooldown: Timer::from_seconds(0.15, TimerMode::Once)
                },
                Weapon {
                    trigger: KeyCode::N,
                    offset: Vec3{ x: -20.0, y: 5.0, z: 0.0 },
                    cooldown: Timer::from_seconds(0.15, TimerMode::Once)
                }
            ]
        )
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..Default::default()
            },
            ..default()
        },
        ProjectileBundle {
            bounds: DespawnBounds::new(1600.0, 900.0),
            speed: ProjectileSpeed(10.0)
        }
    ));
}
