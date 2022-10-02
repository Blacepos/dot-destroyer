use bevy::{prelude::*, window::PresentMode, audio::AudioSink};

use components::*;
mod components;

const MUSIC: &str = "dot_destroyer3-beta00.ogg";

const BULLET_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Dot Destroyer 2 Beta".to_string(),
            width: 800.0,
            height: 600.0,
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
        .add_system(player_shoot)
        .run();
        
}

fn initialize(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(ShipBundle::new(
        meshes,
        materials,
        8.0,
        Color::rgb(0.0, 0.28, 0.95)
    ))
        .insert(Player);
}

fn start_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    audio.play(asset_server.load(MUSIC));
}

/// Checks keyboard input and sets the player's velocity accordingly
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
fn accel_entities(time: Res<Time>, mut query: Query<(&mut Velocity, &Accel)>) {
    for (mut vel, accel) in query.iter_mut() {
        vel.0 += accel.0 * time.delta_seconds();
    }
}

/// Increases the position of each entity by the velocity
fn move_entities(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut tf, vel) in query.iter_mut() {
        tf.translation += vel.0 * time.delta_seconds();
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


