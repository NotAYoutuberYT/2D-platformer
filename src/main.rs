extern crate minifb;

use minifb::{Key, Window, WindowOptions};

mod objects;
use objects::{bounds_contain_point, MovingObject, RectObject, RigidBody, StaticObject, Vector2};

// this is used in main (f64 doesn't implement ord so we have to write this ourselves)
fn f64_min(f1: f64, f2: f64) -> f64 {
    match f1 < f2 {
        true => f1,
        false => f2,
    }
}

//
// constants
//

// window stuff
const WINDOW_WIDTH: usize = 0xff * 4;
const WINDOW_HEIGHT: usize = 0xff * 3;
const FPS: f64 = 144.0;

// player stuff
const PLAYER_WALKING_SPEED: f64 = 2.9;
const PLAYER_RUNNING_SPEED: f64 = 3.6;
const PLAYER_AIR_ACCELL_RATIO: f64 = 0.1;

// physics stuff
const FRICTION_GROUND: f64 = 0.7;
const FRICTION_AIR: f64 = 0.08;

// stuff pertaining to keeping the camera focused on the player
const PERCENT_SCREEN_PLAYER_IN_X: f64 = 18.0;
const PERCENT_SCREEN_PLAYER_IN_Y: f64 = 17.5;
const PLAYER_FOCUS_X_OFFSET: f64 = 0.0;
const PLAYER_FOCUS_Y_OFFSET: f64 = -230.0;
const CAMERA_MOVING_EASING_X: f64 = 1.0 / 750.0;
const CAMERA_MOVING_EASING_Y: f64 = 1.0 / 1300.0;

// jump stuff
const JUMP_FORCE: f64 = 5.0;
const JUMP_BUFFER_HUNDRETHSECS: f64 = 0.0005;

// gravity
const GRAVITY_MOVING_UP: f64 = -1.0 / 7.8;
const GRAVITY_MOVING_DOWN: f64 = -1.0 / 4.5;
const VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT: f64 = -1.0 / 2.5;

// increasing this may increase performance on low fps
// but will make player snap to the edges of platforms
const COLLISION_DEPTH_BASE: f64 = 3.5;
const COLLISION_MAX_LOOPS: u32 = 12;

//
// don't touch these constants
//

const MIN_X_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    - PERCENT_SCREEN_PLAYER_IN_X / 200.0 * WINDOW_WIDTH as f64
    + PLAYER_FOCUS_X_OFFSET;
const MAX_X_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    + PERCENT_SCREEN_PLAYER_IN_X / 200.0 * WINDOW_WIDTH as f64
    + PLAYER_FOCUS_X_OFFSET;
const MIN_Y_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    - PERCENT_SCREEN_PLAYER_IN_Y / 200.0 * WINDOW_HEIGHT as f64
    + PLAYER_FOCUS_Y_OFFSET;
const MAX_Y_FROM_CAMERA: f64 = WINDOW_WIDTH as f64 / 2.0
    + PERCENT_SCREEN_PLAYER_IN_Y / 200.0 * WINDOW_HEIGHT as f64
    + PLAYER_FOCUS_Y_OFFSET;

const FRAME_LIMIT_MILLIS: u64 = (1000.0 / FPS) as u64;

//
// main
//

fn main() {
    let mut player = RigidBody {
        center: Vector2::new(400.0, 300.0),
        width: 20.0,
        height: 40.0,

        velocity: Vector2::new(0.0, 0.0),
        static_friction: false,
    };

    let static_objects = [
        StaticObject {
            center: Vector2::new(510.0, -50.0),
            width: 1750.0,
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

    let mut moving_objects = [
        MovingObject::new(
            Vector2::new(300.0, 245.0),
            Vector2::new(265.0, 575.0),
            80.0,
            35.0,
            150.0,
            false,
        ),
        MovingObject::new(
            Vector2::new(575.0, 390.0),
            Vector2::new(775.0, 545.0),
            100.0,
            35.0,
            150.0,
            false,
        ),
        MovingObject::new(
            Vector2::new(200.0, 770.0),
            Vector2::new(-1000.0, 60.0),
            150.0,
            30.0,
            360.0,
            false,
        ),
    ];

    // our window :)
    let mut window = Window::new(
        "Platformer - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|error| {
        panic!("Error opening window: {}", error);
    });

    // configure the window
    window.limit_update_rate(Some(std::time::Duration::from_millis(FRAME_LIMIT_MILLIS)));
    window.set_position(20, 20);

    // this will store the location of the bottom left corner of the rendring window
    let mut camera_bottom_left: Vector2 =
        Vector2::new(player.center.x - WINDOW_WIDTH as f64 / 2.0, 0.0);

    // this will be where we write out pixel values
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // how long each frame takes (in hundreths of a seconds)
    let mut frame_time: f64 = 0.0;

    // jump buffers make movement feel a little better
    let mut jump_buffer: f64 = 0.0;

    // cache object bounds
    let static_object_bounds: Vec<(f64, f64, f64, f64)> = static_objects
        .iter()
        .map(|object| object.bounds())
        .collect();
    let mut player_bounds: (f64, f64, f64, f64);

    // this prevent "bouncing" on downward moving platforms
    let mut stuck_platform: Option<MovingObject> = None;

    // these are used in physics so it's in scope outside of game loop
    let mut on_object: bool = false;

    //
    // game loop
    //

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // used to measure the frame time
        let frame_start = std::time::Instant::now();

        //
        // player movement and velocity
        //

        // this is where the player's acceleration is stored
        let mut player_acceleration: Vector2 = Vector2::new(0.0, 0.0);

        // configure vertical acceleration (gravity)
        player_acceleration.y = match player.velocity.y <= 0.0 {
            false => GRAVITY_MOVING_UP,
            true => GRAVITY_MOVING_DOWN,
        };

        // configure horizontal acceleration (movement)
        let mut current_accel_speed = match window.is_key_down(Key::LeftShift) {
            true => PLAYER_RUNNING_SPEED,
            false => PLAYER_WALKING_SPEED,
        };
        if !on_object {
            current_accel_speed *= PLAYER_AIR_ACCELL_RATIO;
        }

        if window.is_key_down(Key::D) {
            player_acceleration.x += current_accel_speed;
        }
        if window.is_key_down(Key::A) {
            player_acceleration.x -= current_accel_speed;
        }

        // configure horizontal acceleration (crude friction)
        let current_friction = f64_min(
            player.velocity.x.abs(),
            match on_object {
                true => FRICTION_GROUND * player.velocity.x.abs(),
                false => FRICTION_AIR * player.velocity.x.abs(),
            },
        );

        if player.velocity.x < 0.0 {
            player_acceleration.x += current_friction;
        } else {
            player_acceleration.x -= current_friction;
        }

        if window.is_key_pressed(Key::Q, minifb::KeyRepeat::No) {
            println!("current_friction: {current_friction:?}, current_accel_speed: {current_accel_speed:?}, vel x: {:?}, acc x: {:?}", player.velocity.x, player_acceleration.x);
        }

        // move the player (we integrate the player's movement instead of approximating
        // to make the physics continuous and therefore frame-independent)
        let movement_vector: Vector2 = Vector2::add(
            &Vector2::multiply(&player_acceleration, frame_time * frame_time / 2.0),
            &Vector2::multiply(&player.velocity, frame_time),
        );
        player.move_by(&movement_vector);

        // update velocity
        Vector2::multiply(&player_acceleration, frame_time).add_to(&mut player.velocity);

        //
        // moving platform stuff
        //

        // update moving platforms
        for moving_object in &mut moving_objects {
            (moving_object.update(frame_time));
        }

        // move into the platform we're stuck to if it exists
        if let Some(mut stuck_obj) = stuck_platform {
            if player.collides_with_x(&stuck_obj) {
                stuck_obj.update(frame_time);
                player.center.y =
                    stuck_obj.bounds().3 + player.height / 2.0 - COLLISION_DEPTH_BASE / 2.0;
            }
        }

        stuck_platform = None;

        //
        // collision handling
        //

        on_object = false;
        let mut under_object: bool = false;
        let mut on_side: bool = false;

        // cache player bounds for physics
        player_bounds = player.bounds();

        for object in &moving_objects {
            let bounds = object.bounds();

            // if we collide with the object, decide the best
            // way to move ourselves outside of the object
            for j in 0..COLLISION_MAX_LOOPS {
                if !player.collides_with(object) {
                    break;
                }

                // if we collide with the object, decide the best
                // way to move ourselves outside of the object
                if player_bounds.0 >= bounds.1 - COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.x = bounds.1 + player.width / 2.0;

                    on_side = true;
                } else if player_bounds.1 <= bounds.0 + COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.x = bounds.0 - player.width / 2.0;

                    on_side = true;
                }
                // if we're on top of a moving object, move with it
                else if player_bounds.2 >= bounds.3 - COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.x += object.prev_move.x;

                    // + 0.001 fixes bugs, doesn't affect movement because player will stick to the platform
                    player.center.y = bounds.3 + player.height / 2.0 + 0.001;

                    stuck_platform = Some(object.clone());

                    on_object = true;
                } else if player_bounds.3 <= bounds.2 + COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.y = bounds.2 - player.height / 2.0;

                    under_object = true;
                }
            }
        }

        for i in 0..static_objects.len() {
            let bounds = static_object_bounds[i];

            // this loop is here so that we can't phase through
            // an object if we go fast enough, but still keep
            // the area where the player is considered colliding
            // on multiple sides of the object as small as possible
            for j in 0..COLLISION_MAX_LOOPS {
                if !player.collides_with(&static_objects[i]) {
                    break;
                }

                // if we collide with the object, decide the best
                // way to move ourselves outside of the object
                if player_bounds.0 >= bounds.1 - COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.x = bounds.1 + player.width / 2.0;

                    on_side = true;
                } else if player_bounds.1 <= bounds.0 + COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.x = bounds.0 - player.width / 2.0;

                    on_side = true;
                } else if player_bounds.2 >= bounds.3 - COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.y = bounds.3 + player.height / 2.0;

                    on_object = true;
                } else if player_bounds.3 <= bounds.2 + COLLISION_DEPTH_BASE * (1 << j) as f64 {
                    player.center.y = bounds.2 - player.height / 2.0;

                    under_object = true;
                }
            }
        }

        //
        // final physics before rendering graphics
        //

        jump_buffer -= frame_time;

        if on_side {
            player.velocity.x = 0.0;
        }

        // if space is pressed, start jump buffer
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            jump_buffer = JUMP_BUFFER_HUNDRETHSECS;
        }

        if on_object && jump_buffer > 0.0 {
            player.velocity.y = JUMP_FORCE;
            jump_buffer = 0.0;
            stuck_platform = None; // if we jump, unstick ourselves
        } else if on_object {
            player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        } else if under_object {
            player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        }

        // recache bounds for graphics
        player_bounds = player.bounds();

        // respawn (temporary for prototype)
        if player.center.y < -20.0 {
            player.center = Vector2::new(400.0, 300.0);
        }

        //
        // keep the camera centered on the player
        //

        if player.center.x - camera_bottom_left.x < MIN_X_FROM_CAMERA {
            camera_bottom_left.x -= (player.center.x - camera_bottom_left.x - MIN_X_FROM_CAMERA)
                * (player.center.x - camera_bottom_left.x - MIN_X_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - camera_bottom_left.x > MIN_X_FROM_CAMERA {
                camera_bottom_left.x = player.center.x - MIN_X_FROM_CAMERA;
            }
        } else if player.center.x - camera_bottom_left.x > MAX_X_FROM_CAMERA {
            camera_bottom_left.x += (player.center.x - camera_bottom_left.x - MAX_X_FROM_CAMERA)
                * (player.center.x - camera_bottom_left.x - MAX_X_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - camera_bottom_left.x < MAX_X_FROM_CAMERA {
                camera_bottom_left.x = player.center.x - MAX_X_FROM_CAMERA;
            }
        }

        if player.center.y - camera_bottom_left.y < MIN_Y_FROM_CAMERA {
            camera_bottom_left.y -= (player.center.y - camera_bottom_left.y - MIN_Y_FROM_CAMERA)
                * (player.center.y - camera_bottom_left.y - MIN_Y_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - camera_bottom_left.y > MIN_Y_FROM_CAMERA {
                camera_bottom_left.y = player.center.y - MIN_Y_FROM_CAMERA;
            }
        } else if player.center.y - camera_bottom_left.y > MAX_Y_FROM_CAMERA {
            camera_bottom_left.y += (player.center.y - camera_bottom_left.y - MAX_Y_FROM_CAMERA)
                * (player.center.y - camera_bottom_left.y - MAX_Y_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - camera_bottom_left.y < MAX_Y_FROM_CAMERA {
                camera_bottom_left.y = player.center.y - MAX_Y_FROM_CAMERA;
            }
        }

        //
        // graphics rendering
        //

        for x in 0..WINDOW_WIDTH {
            for y in 0..WINDOW_HEIGHT {
                let rgb: u32;
                let world_point = Vector2::new(
                    camera_bottom_left.x + x as f64,
                    camera_bottom_left.y + (WINDOW_HEIGHT - y) as f64,
                );

                let mut static_object_collision: bool = false;
                let mut moving_object_collision: bool = false;

                // determine collision with static objects
                for bounds in &static_object_bounds {
                    if bounds_contain_point(&world_point, &bounds) {
                        static_object_collision = true;
                    }
                }

                // determine collision with moving objects
                for moving_object in &moving_objects {
                    if moving_object.contains_point(&world_point) {
                        moving_object_collision = true;
                    }
                }

                if bounds_contain_point(&world_point, &player_bounds) {
                    rgb = 0xff0000;
                } else if moving_object_collision {
                    rgb = 0xff00;
                } else if static_object_collision {
                    rgb = 0xff;
                } else {
                    rgb = 0x200020;
                }

                window_buffer[y * WINDOW_WIDTH + x] = rgb;
            }
        }

        // update our window with our pixel values
        window
            .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap_or_else(|error| {
                panic!("Error updating window: {}", error);
            });

        // update how long the frame took
        frame_time = frame_start.elapsed().as_micros() as f64 / 10000.0;
    }
}
