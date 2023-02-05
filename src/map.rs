use super::{
    camera::Rgb,
    constants::{
        CHECKPOINT_COLOR, GOAL_COLOR, MOVING_PLATFORM_INDICATOR_COLOR,
        MOVING_PLATFORM_INDICATOR_RADIUS, PLAYER_HEIGHT, PLAYER_WIDTH,
    },
    objects::{Circle, MovingObject, RigidBody, StaticObject, Vector2},
};

pub struct Checkpoint {
    pub indicator: Circle,
    pub respawn: RigidBody,
}

impl Checkpoint {
    pub fn new(indicator: Circle, player_center: Vector2) -> Checkpoint {
        Checkpoint {
            indicator,
            respawn: RigidBody {
                center: player_center,
                width: PLAYER_WIDTH,
                height: PLAYER_HEIGHT,
                velocity: Vector2::new(0.0, 0.0),
            },
        }
    }
}

pub struct Map {
    pub static_objects: Vec<StaticObject>,
    pub moving_objects: Vec<MovingObject>,

    // circles
    pub moving_object_indicators: Vec<Circle>,
    pub checkpoints: Vec<Checkpoint>,
    pub goal: Circle,

    /// the rigidbody the player will
    /// be set to when it respawns
    pub player_respawn: RigidBody,

    // the player
    pub player: RigidBody,

    /// if the player goes below this point, they respawn
    pub lowest_point: f64,
}

impl Map {
    pub fn new() -> Map {
        Map {
            static_objects: Vec::new(),
            moving_objects: Vec::new(),

            moving_object_indicators: Vec::new(),
            checkpoints: Vec::new(),
            goal: Circle::new(&Vector2::new(0.0, 0.0), 0.0, Rgb::new(0, 0, 0)),

            player_respawn: RigidBody::new(),
            player: RigidBody::new(),

            lowest_point: 0.0,
        }
    }

    /// loads the map with the level provided
    pub fn load_map(&mut self, level: u32) {
        *self = Map::new();

        // determine the level and put the
        // correct map in the this object
        match level {
            1 => {
                self.static_objects = vec![
                    StaticObject::new(Vector2::new(180.0, -520.0), 440.0, 1000.0),
                    StaticObject::new(Vector2::new(650.0, -520.0), 300.0, 1100.0),
                ];

                self.player_respawn = RigidBody {
                    center: Vector2::new(0.0, 0.0),
                    width: PLAYER_WIDTH,
                    height: PLAYER_HEIGHT,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.goal = Circle::new(&Vector2::new(700.0, 100.0), 20.0, GOAL_COLOR);

                self.lowest_point = -200.0;
            }

            2 => {
                self.static_objects = vec![
                    StaticObject::new(Vector2::new(180.0, -520.0), 440.0, 1000.0),
                    StaticObject::new(Vector2::new(1125.0, -500.0), 440.0, 1000.0),
                ];

                self.moving_objects = vec![MovingObject::new(
                    Vector2::new(500.0, -32.5),
                    Vector2::new(740.0, 20.0),
                    100.0,
                    26.0,
                    170.0,
                )];

                self.player_respawn = RigidBody {
                    center: Vector2::new(0.0, 0.0),
                    width: PLAYER_WIDTH,
                    height: PLAYER_HEIGHT,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.goal = Circle::new(&Vector2::new(1250.0, 100.0), 20.0, GOAL_COLOR);

                self.lowest_point = -160.0;
            }

            3 => {
                self.static_objects = vec![
                    StaticObject::new(Vector2::new(200.0, -500.0), 400.0, 1000.0),
                    StaticObject::new(Vector2::new(325.0, 340.0), 150.0, 100.0),
                    StaticObject::new(Vector2::new(50.0, 370.0), 150.0, 100.0),
                    StaticObject::new(Vector2::new(-225.0, 400.0), 150.0, 100.0),
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
                    width: PLAYER_WIDTH,
                    height: PLAYER_HEIGHT,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.goal = Circle::new(&Vector2::new(-480.0, 530.0), 20.0, GOAL_COLOR);

                self.lowest_point = -150.0;
            }

            4 => {
                self.static_objects = vec![
                    StaticObject::new(Vector2::new(100.0, -500.0), 400.0, 1000.0),
                    StaticObject::new(Vector2::new(480.0, 10.0), 100.0, 100.0),
                    StaticObject::new(Vector2::new(100.0, 250.0), 200.0, 90.0),
                    StaticObject::new(Vector2::new(-150.0, 300.0), 110.0, 110.0),
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
                        Vector2::new(300.0, 470.0),
                        100.0,
                        30.0,
                        200.0,
                    ),
                ];

                self.player_respawn = RigidBody {
                    center: Vector2::new(0.0, 0.0),
                    width: PLAYER_WIDTH,
                    height: PLAYER_HEIGHT,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.goal = Circle::new(&Vector2::new(300.0, 520.0), 20.0, GOAL_COLOR);

                self.lowest_point = -120.0;
            }

            5 => {
                self.static_objects = vec![
                    StaticObject::new(Vector2::new(0.0, -800.0), 600.0, 1600.0),
                    StaticObject::new(Vector2::new(500.0, 300.0), 60.0, 600.0),
                    StaticObject::new(Vector2::new(700.0, 300.0), 60.0, 600.0),
                    StaticObject::new(Vector2::new(900.0, 300.0), 60.0, 600.0),
                    StaticObject::new(Vector2::new(1100.0, 35.0), 70.0, 70.0),
                    StaticObject::new(Vector2::new(1300.0, 60.0), 100.0, 80.0),
                ];

                self.moving_objects = vec![MovingObject::new(
                    Vector2::new(900.0, -100.0),
                    Vector2::new(900.0, -12.5),
                    1000.0,
                    25.0,
                    100.0,
                )];

                self.player_respawn = RigidBody {
                    center: Vector2::new(0.0, 0.0),
                    width: PLAYER_WIDTH,
                    height: PLAYER_HEIGHT,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.goal = Circle::new(&Vector2::new(1480.0, 200.0), 20.0, GOAL_COLOR);

                self.lowest_point = -500.0;
            }

            6 => {
                self.static_objects = vec![
                    StaticObject::new(Vector2::new(100.0, -520.0), 400.0, 1000.0),
                    StaticObject::new(Vector2::new(80.0, 310.0), 120.0, 100.0),
                    StaticObject::new(Vector2::new(-200.0, 270.0), 120.0, 100.0),
                    StaticObject::new(Vector2::new(0.0, 590.0), 100.0, 70.0),
                    StaticObject::new(Vector2::new(270.0, 610.0), 120.0, 100.0),
                ];

                self.moving_objects = vec![
                    MovingObject::new(
                        Vector2::new(480.0, -50.0),
                        Vector2::new(340.0, 300.0),
                        100.0,
                        30.0,
                        180.0,
                    ),
                    MovingObject::new(
                        Vector2::new(-400.0, 340.0),
                        Vector2::new(-320.0, 600.0),
                        100.0,
                        30.0,
                        100.0,
                    ),
                ];

                self.player_respawn = RigidBody {
                    center: Vector2::new(0.0, 0.0),
                    width: PLAYER_WIDTH,
                    height: PLAYER_HEIGHT,

                    velocity: Vector2::new(0.0, 0.0),
                };

                self.checkpoints = vec![Checkpoint::new(
                    Circle::new(&Vector2::new(80.0, 400.0), 15.0, CHECKPOINT_COLOR),
                    Vector2::new(80.0, 360.0 + PLAYER_HEIGHT / 2.0),
                )];

                self.goal = Circle::new(&Vector2::new(440.0, 765.0), 20.0, GOAL_COLOR);

                self.lowest_point = -250.0;
            }

            _ => panic!("Map.load_map given improper level number"),
        }

        // set the starting player to the default player respawn
        self.player = self.player_respawn;

        // put a moving platform end indicator
        // at the end of all moving objects
        self.moving_objects.iter().for_each(|object| {
            self.moving_object_indicators.push(Circle::new(
                &object.start_pos(),
                MOVING_PLATFORM_INDICATOR_RADIUS,
                MOVING_PLATFORM_INDICATOR_COLOR,
            ));
            self.moving_object_indicators.push(Circle::new(
                &object.end_pos(),
                MOVING_PLATFORM_INDICATOR_RADIUS,
                MOVING_PLATFORM_INDICATOR_COLOR,
            ));
        })
    }
}