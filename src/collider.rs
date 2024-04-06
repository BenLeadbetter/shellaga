use bevy::math::f32::Vec2;
use bevy::transform::components::GlobalTransform;

#[derive(bevy::ecs::component::Component, Debug, Default, PartialEq)]
pub struct Collider(Vec2);

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self(Vec2::new(width, height))
    }
}

impl std::ops::Deref for Collider {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Collider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(dead_code)]
pub fn collide(c1: (&Collider, &GlobalTransform), c2: (&Collider, &GlobalTransform)) -> bool {
    use bevy::math::Vec3Swizzles;

    let top_left1 = c1.1.translation().xy();
    let bottom_right1 = c1.1.translation().xy() + **c1.0;

    let top_left2 = c2.1.translation().xy();
    let bottom_right2 = c2.1.translation().xy() + **c2.0;

    let top_left1_inside_2 = top_left1.x > top_left2.x
        && top_left1.x < bottom_right2.x
        && top_left1.y > top_left2.y
        && top_left1.y < bottom_right2.y;

    let bottom_right1_inside_2 = bottom_right1.x > top_left2.x
        && bottom_right1.x < bottom_right2.x
        && bottom_right1.y > top_left2.y
        && bottom_right1.y < bottom_right2.y;

    top_left1_inside_2 || bottom_right1_inside_2
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::f32::Vec3;
    use bevy::transform::components::Transform;

    #[test]
    fn no_collision() {
        //  _   _
        // |_| |_|
        //
        assert!(!collide(
            (&Collider(Vec2::new(1.0, 1.0)), &GlobalTransform::default()),
            (
                &Collider(Vec2::new(1.0, 1.0)),
                &Transform::from_translation(Vec3::new(2.0, 0.0, 0.0)).into()
            ),
        ));
    }

    #[test]
    fn collision() {
        //  ________
        // |        |
        // |     ___|____
        // |    |   |   |
        // |____|___|   |
        //      |       |
        //      |_______|
        //
        assert!(collide(
            (&Collider(Vec2::new(1.0, 1.0)), &GlobalTransform::default()),
            (
                &Collider(Vec2::new(1.0, 1.0)),
                &Transform::from_translation(Vec3::new(0.5, 0.5, 0.0)).into()
            ),
        ));
    }
}
