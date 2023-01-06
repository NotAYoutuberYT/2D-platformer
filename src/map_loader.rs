use super::objects::{MovingObject, StaticObject, Vector2};

// for now, this just returns a hard-coded prototype map
pub fn load_map(static_objects: &mut Vec<StaticObject>, moving_objects: &mut Vec<MovingObject>) {
    *static_objects = vec![
        StaticObject {
            center: Vector2::new(510.0, -50.0),
            width: 1000.0,
            height: 400.0,
        },
        StaticObject {
            center: Vector2::new(430.0, 185.0),
            width: 80.0,
            height: 80.0,
        },
        StaticObject {
            center: Vector2::new(360.0, 650.0),
            width: 50.0,
            height: 225.0,
        },
        StaticObject {
            center: Vector2::new(410.0, 562.5),
            width: 50.0,
            height: 50.0,
        },
        StaticObject {
            center: Vector2::new(406.5, 665.0),
            width: 43.0,
            height: 20.0,
        },
        StaticObject {
            center: Vector2::new(-1000.0, 500.0),
            width: 50.0,
            height: 50.0,
        },
        StaticObject {
            center: Vector2::new(-1125.0, 500.0),
            width: 50.0,
            height: 50.0,
        },
        StaticObject {
            center: Vector2::new(-1025.0, 300.0),
            width: 200.0,
            height: 50.0,
        },
    ];

    *moving_objects = vec![
        MovingObject::new(
            Vector2::new(300.0, 245.0),
            Vector2::new(265.0, 565.0),
            80.0,
            35.0,
            150.0,
            false,
        ),
        MovingObject::new(
            Vector2::new(575.0, 390.0),
            Vector2::new(735.0, 545.0),
            100.0,
            35.0,
            150.0,
            false,
        ),
        MovingObject::new(
            Vector2::new(175.0, 770.0),
            Vector2::new(-1000.0, 60.0),
            200.0,
            30.0,
            360.0,
            false,
        ),
    ];
}
