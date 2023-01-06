use super::objects::{MovingObject, StaticObject, Vector2};

// for now, this just returns a hard-coded prototype map
pub fn load_map(static_objects: &mut Vec<StaticObject>, moving_objects: &mut Vec<MovingObject>) {
    *static_objects = vec![
        StaticObject {
            center: Vector2::new(100.0, -500.0),
            width: 400.0,
            height: 1000.0,
        },
        StaticObject {
            center: Vector2::new(480.0, 10.0),
            width: 100.0,
            height: 100.0,
        },
        StaticObject {
            center: Vector2::new(100.0, 190.0),
            width: 200.0,
            height: 90.0,
        },
    ];

    *moving_objects = vec![MovingObject::new(
        Vector2::new(365.0, 75.0),
        Vector2::new(365.0, 200.0),
        120.0,
        30.0,
        120.0,
        false,
    )];
}
