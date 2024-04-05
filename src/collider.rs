use bevy::math::f32::Vec2;
use bevy::transform::components::Transform;

#[derive(bevy::ecs::component::Component, Debug, Default, PartialEq)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
}

#[allow(dead_code)]
pub struct ColliderWithTransform<'a, 'b> {
    pub collider: &'a Collider,
    pub transform: &'b Transform,
}

#[allow(dead_code)]
pub fn collide(c1: &ColliderWithTransform, c2: &ColliderWithTransform) -> bool {
    use bevy::math::Vec3Swizzles;

    let top_left1 = c1.transform.translation.xy() + c1.collider.offset;
    let bottom_right1 = c1.transform.translation.xy() + c1.collider.offset + c1.collider.size;

    let top_left2 = c2.transform.translation.xy() + c2.collider.offset;
    let bottom_right2 = c2.transform.translation.xy() + c2.collider.offset + c2.collider.size;

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

    #[test]
    fn no_collision() {
        //  _   _
        // |_| |_|
        //
        assert!(!collide(
            &ColliderWithTransform {
                collider: &Collider {
                    size: Vec2::new(1.0, 1.0),
                    ..Default::default()
                },
                transform: &Transform::default(),
            },
            &ColliderWithTransform {
                collider: &Collider {
                    size: Vec2::new(1.0, 1.0),
                    ..Default::default()
                },
                transform: &Transform::from_translation(Vec3::new(2.0, 0.0, 0.0)),
            }
        ))
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
            &ColliderWithTransform {
                collider: &Collider {
                    size: Vec2::new(1.0, 1.0),
                    ..Default::default()
                },
                transform: &Transform::default(),
            },
            &ColliderWithTransform {
                collider: &Collider {
                    size: Vec2::new(1.0, 1.0),
                    ..Default::default()
                },
                transform: &Transform::from_translation(Vec3::new(0.5, 0.5, 0.0)),
            }
        ))
    }
}
