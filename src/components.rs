use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};


#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct FromPlayer;


#[derive(Component)]
pub struct FromEnemy;


#[derive(Component)]
pub struct ShipStats {
    pub radius: f32,
    pub base_accel: f32,
    pub max_speed: f32,
    pub damage: f32,
    pub base_health: f32,
    pub color: Color
}


#[derive(Component)]
pub struct Bullet;


#[derive(Component)]
pub struct BulletStats {
    pub radius: f32,
    pub damage: f32
}


#[derive(Component)]
pub struct Gun {
    pub timer: Timer,
    pub is_shooting: bool,
}

#[derive(Component)]
pub struct Velocity(pub Vec3);


#[derive(Component)]
pub struct Accel(pub Vec3);


#[derive(Component)]
pub struct AimingAt(pub Vec3);

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool
}


#[derive(Bundle)]
pub struct ShipBundle {
    pub ship_stats: ShipStats,
    pub vel: Velocity,
    pub accel: Accel,
    pub gun: Gun,
    pub aiming_at: AimingAt,
    pub movable: Movable,

    #[bundle]
    pub body: MaterialMesh2dBundle<ColorMaterial>
}

impl ShipBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        radius: f32,
        color: Color,
        z_offset: f32
    ) -> Self {
        ShipBundle {
            ship_stats: ShipStats {
                radius: radius,
                base_accel: 1000.0,
                damage: 1.0,
                base_health: 100.0,
                color: color,
                max_speed: 400.0,
            },
            vel: Velocity(Vec3::ZERO),
            accel: Accel(Vec3::ZERO),
            gun: Gun {
                timer: Timer::new(Duration::from_millis(500), false),
                is_shooting: false
            },
            body: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(radius))).into(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, z_offset)),
                material: materials.add(ColorMaterial::from(color)),
                ..default()
            },
            aiming_at: AimingAt(Vec3::X),
            movable: Movable { auto_despawn: false },
        }
    }

    #[allow(dead_code)]
    pub fn with_firing_rate(mut self, rate: Duration) -> Self {
        self.gun.timer.set_duration(rate);
        self
    }

    #[allow(dead_code)]
    pub fn with_base_accel(mut self, base_accel: f32) -> Self {
        self.ship_stats.base_accel = base_accel;
        self
    }

    #[allow(dead_code)]
    pub fn with_damage(mut self, damage: f32) -> Self {
        self.ship_stats.damage = damage;
        self
    }
    
    #[allow(dead_code)]
    pub fn with_max_speed(mut self, max_speed: f32) -> Self {
        self.ship_stats.max_speed = max_speed;
        self
    }
    
    #[allow(dead_code)]
    pub fn always_shooting(mut self) -> Self {
        self.gun.is_shooting = true;
        self
    }
}


#[derive(Bundle)]
pub struct BulletBundle {
    vel: Velocity,
    bullet_stats: BulletStats,
    bullet: Bullet,
    movable: Movable,

    #[bundle]
    body: MaterialMesh2dBundle<ColorMaterial>
}

impl BulletBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        radius: f32,
        color: Color,
        damage: f32,
        direction: Vec3,
        position: Vec3
    ) -> Self {
        BulletBundle {
            bullet_stats: BulletStats {
                radius: radius,
                damage: damage,
            },
            vel: Velocity(direction),
            body: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(radius))).into(),
                transform: Transform::from_translation(position),
                material: materials.add(ColorMaterial::from(color)),
                ..default()
            },
            bullet: Bullet,
            movable: Movable { auto_despawn: true },
        }
    }
}
