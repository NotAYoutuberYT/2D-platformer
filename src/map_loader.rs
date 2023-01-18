use super::objects::{MovingObject, StaticObject, Vector2};

pub struct Map {
    pub static_objects: Vec<StaticObject>,
    pub moving_objects: Vec<MovingObject>,
}

impl Map {
    pub fn new() -> Map {
        return Map {
            static_objects: Vec::new(),
            moving_objects: Vec::new(),
        };
    }

    /// fills the map with the level provided
    pub fn load_map(&mut self, level: u32) {
        if level == 1 {
            self.static_objects = vec![
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
                    center: Vector2::new(100.0, 250.0),
                    width: 200.0,
                    height: 90.0,
                },
                StaticObject {
                    center: Vector2::new(-150.0, 300.0),
                    width: 110.0,
                    height: 110.0,
                },
            ];

            self.moving_objects = vec![
                MovingObject::new(
                    Vector2::new(365.0, 100.0),
                    Vector2::new(365.0, 210.0),
                    120.0,
                    30.0,
                    140.0,
                    false,
                ),
                MovingObject::new(
                    Vector2::new(-30.0, 420.0),
                    Vector2::new(300.0, 460.0),
                    100.0,
                    30.0,
                    200.0,
                    false,
                ),
            ];
        }
    }
}
