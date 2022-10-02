use bevy::prelude::Vec3;


/// Finds the future position of a moving target such that it intersects the path of a projectile.
/// Due to the complexity of this function, it should used sparingly.
/// https://gamedev.stackexchange.com/a/25292
pub fn predict(target_pos: Vec3, target_vel: Vec3, proj_pos: Vec3, proj_speed: f32) -> Option<Vec3> {
    let to_target = target_pos - proj_pos;

    let a = target_vel.length_squared() - proj_speed * proj_speed;
    let a2 = 2.0 * a;
    let b = 2.0 * Vec3::dot(target_vel, to_target);
    let c = to_target.length_squared();

    let discrim = b * b - 4.0 * a * c;

    
    if discrim >= 0.0 {
        let p = -b / a2;
        let q = f32::sqrt(discrim) / a2;
    
        let t1 = p + q;
        let t2 = p - q;

        let t = if 0.0 < t2 && t2 < t1 {t2} else {t1};

        Some(target_pos - target_vel * t)
    }
    else {
        None
    }
}