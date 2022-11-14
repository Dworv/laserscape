use bevy::{prelude::*, time::FixedTimestep};

use crate::{MOVE_ACCL, MOVE_DRAG, TIME_STEP, TURN_ACCL, TURN_DRAG};

#[derive(Component)]
pub struct MoveControls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}
#[derive(Default, Component)]
pub struct Velocity(pub Vec3);
#[derive(Default, Component)]
pub struct TurnSpeed(pub f32);
#[derive(Default, Component)]
pub struct Thrust(pub f32);
#[derive(Component)]
pub struct Bounds(pub f32, pub f32, pub f32, pub f32);
impl Bounds {
    pub fn new(h: f32, v: f32) -> Self {
        Self(-h / 2.0, -v / 2.0, h / 2.0, v / 2.0)
    }
}
#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub turn_speed: TurnSpeed,
    pub thrust: Thrust,
    pub bounds: Bounds,
    pub controls: MoveControls,
}

fn apply_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TurnSpeed, &mut Thrust, &MoveControls)>,
) {
    for (mut turn, mut thrust, controls) in query.iter_mut() {
        let mut thrust_mod = 0.0;
        let mut turn_mod = 0.0;
        if keyboard_input.pressed(controls.up) {
            thrust_mod += MOVE_ACCL;
        }
        if keyboard_input.pressed(controls.down) {
            thrust_mod -= MOVE_ACCL;
        }
        if keyboard_input.pressed(controls.right) {
            turn_mod -= TURN_ACCL;
        }
        if keyboard_input.pressed(controls.left) {
            turn_mod += TURN_ACCL;
        }
        turn.0 += turn_mod;
        thrust.0 = thrust_mod;
    }
}

fn drag_turning(mut query: Query<&mut TurnSpeed>) {
    for mut turn in query.iter_mut() {
        turn.0 /= TURN_DRAG;
    }
}

fn drag_velocity(mut query: Query<&mut Velocity>) {
    for mut velo in query.iter_mut() {
        velo.0 /= MOVE_DRAG
    }
}

fn calculate_velocity(mut query: Query<(&mut Velocity, &Transform, &Thrust)>) {
    for (mut velo, trans, thrust) in query.iter_mut() {
        let rotation = trans.rotation * Vec3::Y;
        let velo_change = thrust.0 * rotation;
        velo.0 += velo_change;
    }
}

fn update_rotation(mut query: Query<(&TurnSpeed, &mut Transform)>) {
    for (turn, mut trans) in query.iter_mut() {
        trans.rotate_z(turn.0 * TIME_STEP);
    }
}

fn update_translation(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velo, mut trans) in query.iter_mut() {
        trans.translation += velo.0;
    }
}

fn keep_in_bounds(mut query: Query<(&Transform, &Bounds, &mut Velocity)>) {
    for (trans, bounds, mut velocity) in query.iter_mut() {
        if trans.translation.x < bounds.0 {
            velocity.0.x = velocity.0.x.abs() / 2.0
        }
        if trans.translation.y < bounds.1 {
            velocity.0.y = velocity.0.y.abs() / 2.0
        }
        if trans.translation.x > bounds.2 {
            velocity.0.x = -(velocity.0.x.abs()) / 2.0
        }
        if trans.translation.y > bounds.3 {
            velocity.0.y = -(velocity.0.y.abs()) / 2.0
        }
    }
}

#[derive(PartialEq, Clone, Hash, Debug, Eq, SystemLabel)]
enum MovementSystem {
    ApplyInput,
    DragTurning,
    DragVelocity,
    UpdateRotation,
    CalculateVelocity,
    KeepInBounds,
    UpdateTranslation,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_input.label(MovementSystem::ApplyInput))
                .with_system(
                    drag_turning
                        .label(MovementSystem::DragTurning)
                        .after(MovementSystem::ApplyInput),
                )
                .with_system(
                    drag_velocity
                        .label(MovementSystem::DragVelocity)
                        .after(MovementSystem::ApplyInput),
                )
                .with_system(
                    update_rotation
                        .label(MovementSystem::UpdateRotation)
                        .after(MovementSystem::DragTurning),
                )
                .with_system(
                    calculate_velocity
                        .label(MovementSystem::CalculateVelocity)
                        .after(MovementSystem::UpdateRotation),
                )
                .with_system(
                    keep_in_bounds
                        .label(MovementSystem::KeepInBounds)
                        .after(MovementSystem::CalculateVelocity),
                )
                .with_system(
                    update_translation
                        .label(MovementSystem::UpdateTranslation)
                        .after(MovementSystem::KeepInBounds),
                ),
        );
    }
}
