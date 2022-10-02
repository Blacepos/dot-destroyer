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


/// The 
#[derive(Component)]
pub struct ShipStats {
    pub radius: f32,
    pub base_accel: f32,
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
pub struct ShootTimer {
    pub timer: Timer,
    pub is_shooting: bool,
}

#[derive(Component)]
pub struct Velocity(pub Vec3);


#[derive(Component)]
pub struct Accel(pub Vec3);


#[derive(Component)]
pub struct AimingAt(pub Vec3);


#[derive(Bundle)]
pub struct ShipBundle {
    pub ship_stats: ShipStats,
    pub vel: Velocity,
    pub accel: Accel,
    pub shoot_timer: ShootTimer,
    pub aiming_at: AimingAt,

    #[bundle]
    pub body: MaterialMesh2dBundle<ColorMaterial>
}

impl ShipBundle {
    pub fn new(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        radius: f32,
        color: Color
    ) -> Self {
        ShipBundle {
            ship_stats: ShipStats {
                radius: radius,
                base_accel: 1000.0,
                damage: 1.0,
                base_health: 100.0,
                color: color
            },
            vel: Velocity(Vec3::ZERO),
            accel: Accel(Vec3::ZERO),
            shoot_timer: ShootTimer {
                timer: Timer::new(Duration::from_millis(500), false),
                is_shooting: false
            },
            body: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::new(radius))).into(),
                transform: Transform::default(),
                material: materials.add(ColorMaterial::from(color)),
                ..default()
            },
            aiming_at: AimingAt(Vec3::X),
        }
    }

    pub fn with_firing_rate(&mut self, rate: Duration) -> &mut Self {
        self.shoot_timer.timer.set_duration(rate);
        self
    }

    pub fn with_speed(&mut self, speed: f32) -> &mut Self {
        self.ship_stats.base_accel = speed;
        self
    }

    pub fn with_damage(&mut self, damage: f32) -> &mut Self {
        self.ship_stats.damage = damage;
        self
    }
    
}


#[derive(Bundle)]
pub struct BulletBundle {
    vel: Velocity,
    bullet_stats: BulletStats,
    bullet: Bullet,

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
            bullet: Bullet
        }
    }
}
