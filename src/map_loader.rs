use crate::constants::MOVING_PLATFORM_INDICATOR_COLOR;

use super::objects::{Circle, MovingObject, RigidBody, StaticObject, Vector2};

pub struct Map {
    pub static_objects: Vec<StaticObject>,
    pub moving_objects: Vec<MovingObject>,
    pub circles: Vec<Circle>,

    /// the rigidbody the player will
    /// be set to when it respawns
    pub player_respawn: RigidBody,

    // the player
    pub player: RigidBody,

    /// if the player goes below this point, they rewspawn
    pub lowest_point: f64,
}

impl Map {
    pub fn new() -> Map {
        return Map {
            static_objects: Vec::new(),
            moving_objects: Vec::new(),
            circles: Vec::new(),

            player_respawn: RigidBody::new(),
            player: RigidBody::new(),

            lowest_point: 0.0,
        };
    }

    /// loads the map with the level provided
    pub fn load_map(&mut self, level: u32) {
        *self = Map::new();


        // determine the level and put the
        // correct map in the this object
        match level {
            1 => {
                self.static_objects = vec![
                    StaticObject {
                        center: Vector2::new(180.0, -520.0),
                        width: 440.0,
                        height: 1000.0,
                    },
                    StaticObject {
                        center: Vector2::new(650.0, -520.0),
                        width: 300.0,
                        height: 1100.0,
                    },
                ];

                self.player_respawn = RigidBody {
                    center: Vector2::new(0.0, 0.0),
                    width: 20.0,
                    height: 40.0,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.lowest_point = -120.0;
            }

            2 => {
                self.static_objects = vec![
                    StaticObject {
                        center: Vector2::new(200.0, -500.0),
                        width: 400.0,
                        height: 1000.0,
                    },
                    StaticObject {
                        center: Vector2::new(200.0, 320.0),
                        width: 400.0,
                        height: 100.0,
                    },
                ];

                self.moving_objects = vec![MovingObject::new(
                    Vector2::new(550.0, -50.0),
                    Vector2::new(550.0, 300.0),
                    100.0,
                    30.0,
                    150.0,
                )];

                self.player_respawn = RigidBody {
                    center: Vector2::new(100.0, 0.0),
                    width: 20.0,
                    height: 40.0,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.lowest_point = -150.0;
            }

            3 => {
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
                    ),
                    MovingObject::new(
                        Vector2::new(-30.0, 420.0),
                        Vector2::new(300.0, 460.0),
                        100.0,
                        30.0,
                        200.0,
                    ),
                ];

                self.player_respawn = RigidBody {
                    center: Vector2::new(0.0, 0.0),
                    width: 20.0,
                    height: 40.0,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.lowest_point = -120.0;
            }

            _ => panic!("Map.load_map given improper level number"),
        }

        // set the starting player to the default player respawn
        self.player = self.player_respawn.clone();

        // put a moving platform end indicator
        // at the end of allmoving objects
        self.moving_objects.iter().for_each(|object| {
            self.circles.push(Circle::new(
                &object.start_pos,
                25.0,
                MOVING_PLATFORM_INDICATOR_COLOR,
            ));
            self.circles.push(Circle::new(
                &object.end_pos,
                25.0,
                MOVING_PLATFORM_INDICATOR_COLOR,
            ));
        })
    }
}
