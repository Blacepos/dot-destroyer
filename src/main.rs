use std::time::Duration;

use bevy::{prelude::*, window::PresentMode};

use components::*;
mod components;

const MUSIC: &str = "dot_destroyer3-beta00.ogg";

const BULLET_SPEED: f32 = 400.0;

const ENEMY_COLOR: Color = Color::rgb(0.91, 0.64, 0.0);
const PLAYER_COLOR: Color = Color::rgb(0.0, 0.28, 0.95);

const WIN_SIZE: (f32, f32) = (800.0, 600.0);

const DESPAWN_RADIUS: f32 = WIN_SIZE.0 + 100.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dot Destroyer 2 Beta".to_string(),
            width: WIN_SIZE.0,
            height: WIN_SIZE.1,
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(initialize)
        .add_startup_system(start_music)
        .add_system(handle_move.before(accel_entities))
        .add_system(accel_entities.before(move_entities))
        .add_system(move_entities)
        .add_system(wrap_player.after(move_entities))
        .add_system(tick_shoot_timers)
        .add_system(player_shoot.before(tick_shoot_timers))
        .add_system(enemy_ai_move.before(tick_shoot_timers))
        .run();
        
}

fn initialize(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(ShipBundle::new(
        &mut meshes,
        &mut materials,
        8.0,
        PLAYER_COLOR,
        0.0
    )
        .with_max_speed(f32::INFINITY)
        .with_firing_rate(Duration::from_millis(200))
        .with_base_accel(25.0 * 60.0)
    )
        .insert(Player);

    commands.spawn_bundle(ShipBundle::new(
        &mut meshes,
        &mut materials,
        6.5,
        ENEMY_COLOR,
        1.0
    )
        .with_max_speed(7.0 * 60.0)
    )
        .insert(Enemy);
}

fn start_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    audio.play(asset_server.load(MUSIC));
}

/// Checks keyboard input and sets the player's acceleration accordingly
fn handle_move(kb: Res<Input<KeyCode>>, mut query: Query<(&mut Accel, &ShipStats), With<Player>>) {
    let (mut accel, ship_stats) = query.get_single_mut().expect("Player should exist. handle_move");
    accel.0.x = 0.0;
    accel.0.y = 0.0;
    if kb.pressed(KeyCode::W) {
        accel.0.y += ship_stats.base_accel;
    }
    if kb.pressed(KeyCode::S) {
        accel.0.y += -ship_stats.base_accel;
    }
    if kb.pressed(KeyCode::D) {
        accel.0.x += ship_stats.base_accel;
    }
    if kb.pressed(KeyCode::A) {
        accel.0.x += -ship_stats.base_accel;
    }
}

/// Checks mouse position for player shooting
fn player_shoot(
    windows: Res<Windows>,
    mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut ShootTimer, &mut AimingAt, &Transform), With<Player>>
) {
    let (mut shoot_timer, mut aiming_at, player_tf) = query.get_single_mut()
        .expect("Player should exist. player_shoot");
    let window = windows.get_primary()
        .expect("A primary window should exist. player_shoot");

    // if the mouse is inside the window, have the player aim at it
    if let Some(mpos) = window.cursor_position() {
        let mpos = Vec3 {
            x: mpos.x - window.width() / 2.0,
            y: mpos.y - window.height() / 2.0,
            z: 0.0
        };
        aiming_at.0 = mpos - player_tf.translation;
    }

    if mouse.just_released(MouseButton::Left) {
        shoot_timer.is_shooting = false;
    }
    if mouse.just_pressed(MouseButton::Left) {
        shoot_timer.is_shooting = true;
        shoot_timer.timer.reset();
    }
}

/// Accelerate each enemy towards the player
fn enemy_ai_move(
    mut enemy_query: Query<(&mut Accel, &Transform, &ShipStats, &mut AimingAt), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>
) {
    let player_pos = player_query.get_single().expect("Player should exist. enemy_ai_move").translation;

    for (mut accel, enemy_tf, ship_stats, mut aiming_at) in enemy_query.iter_mut() {
        let enemy_pos = enemy_tf.translation;

        // have the enemy move towards the player
        accel.0 = (player_pos.truncate() - enemy_pos.truncate())
            .extend(0.0)
            .try_normalize()
            .unwrap_or(Vec3::X)
            * ship_stats.base_accel;
        
        aiming_at.0 = player_pos;
    }
}

/// Ticks all ShootTimer components and spawns bullet if timer done
fn tick_shoot_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut ShootTimer, &ShipStats, &Transform, &AimingAt)>
) {
    for (mut timer, ship_stats, ship_tf, aiming_at) in query.iter_mut() {
        if timer.is_shooting {
            timer.timer.tick(time.delta());

            if timer.timer.finished() {
                timer.timer.reset();

                // spawn bullet
                commands.spawn_bundle(BulletBundle::new(
                    &mut meshes,
                    &mut materials,
                    5.0,
                    ship_stats.color,
                    ship_stats.damage,
                    aiming_at.0.try_normalize().unwrap_or(Vec3::X) * BULLET_SPEED,
                    ship_tf.translation
                ));
            }
        }
    }
}

/// Increases the velocity of each entity by the acceleration
fn accel_entities(time: Res<Time>, mut query: Query<(&mut Velocity, &Accel, &ShipStats)>) {
    for (mut vel, accel, ship_stats) in query.iter_mut() {
        vel.0 += accel.0 * time.delta_seconds();
        vel.0 = vel.0.clamp_length_max(ship_stats.max_speed);
    }
}

/// Increases the position of each entity by the velocity
fn move_entities(
    mut commands: Commands,    
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &Movable, Entity)>
) {
    for (mut tf, vel, movable, entity) in query.iter_mut() {
        tf.translation += vel.0 * time.delta_seconds();

        
        if movable.auto_despawn
            && (tf.translation.x > DESPAWN_RADIUS
            || tf.translation.x < -DESPAWN_RADIUS
            || tf.translation.y > DESPAWN_RADIUS
            || tf.translation.y < -DESPAWN_RADIUS) {
            
            commands.entity(entity).despawn();
        }
    }
}

/// Makes the player go l o o p
fn wrap_player(windows: Res<Windows>, mut query: Query<(&mut Transform, &ShipStats), With<Player>>) {
    let (mut pos, ship_info) = query.get_single_mut().expect("Player should exist. wrap_player");
    let window = windows.get_primary().expect("A primary window should exist. wrap_player");

    let t = pos.translation;

    let w = window.width();
    let w2 = w / 2.0; // save exactly 1 fp division lol
    let h = window.height();
    let h2 = h / 2.0;
    let r = ship_info.radius;
    let r2 = r * 2.0;

    pos.translation = Vec3 {
        x: f32::rem_euclid(t.x + w2 + r, w + r2) - w2 - r,
        y: f32::rem_euclid(t.y + h2 + r, h + r2) - h2 - r,
        z: t.z
    }
}

