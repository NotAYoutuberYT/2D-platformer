/* This project uses a library (known as a crate in Rust) called minifb. It is
owned by Daniel Collin. It can be found on crates.io at https://crates.io/crates/minifb
or on GitHub at https://github.com/emoon/rust_minifb. To use in a project, put the line
of code "minifb = "0.23" into the cargo.toml file of a cargo-initilized project. This
crate is what allows me to open a window and write rgb values to each pixel of the window. All
code for representing objects, rendring those objects, and performing physics is written by Bryce Holland. */
extern crate minifb;
use minifb::{Key, Window, WindowOptions};

mod objects;
use objects::{
    bounds_contain_point, CollisionStates, MovingObject, RectObject, RigidBody, StaticObject,
    Vector2,
};

mod camera;
use camera::Camera;

mod constants;
use constants::*; // all my constants have very specific names, so I'm comfortable doing this

// this is the function we use to render the game
fn render_game(
    world_point: Vector2,
    player_bounds: &(f64, f64, f64, f64),
    is_sprinting: bool,
    static_object_bounds: &[(f64, f64, f64, f64)],
    moving_object_bounds: &Vec<(f64, f64, f64, f64)>,
) -> u32 {
    let rgb: u32;

    let mut player_collision: bool = false;
    let mut static_object_collision: bool = false;
    let mut moving_object_collision: bool = false;

    // determine collision with player
    if bounds_contain_point(&world_point, player_bounds) {
        player_collision = true;
    }

    // determine collision with static objects
    static_object_bounds.iter().for_each(|bounds| {
        if bounds_contain_point(&world_point, bounds) {
            static_object_collision = true;
        }
    });

    // determine collision with moving objects
    moving_object_bounds.iter().for_each(|bounds| {
        if bounds_contain_point(&world_point, bounds) {
            moving_object_collision = true;
        }
    });

    if player_collision && is_sprinting {
        rgb = SPRINTING_PLAYER_COLOR;
    } else if player_collision {
        rgb = NORMAL_PLAYER_COLOR;
    } else if moving_object_collision {
        rgb = MOVING_OBJECT_COLOR;
    } else if static_object_collision {
        rgb = STATIC_OBJECT_COLOR;
    } else {
        rgb = BACKGROUND_COLOR;
    }

    rgb
}

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

    let mut moving_objects = [
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
    let mut camera = Camera::new(player.center.x - WINDOW_WIDTH as f64 / 2.0, 0.0);

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

    // this is where we'll store our active collision
    let mut collision = CollisionStates::NoCollision;

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
        let mut current_accel_speed =
            match window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift) {
                true => PLAYER_RUNNING_ACCEL,
                false => PLAYER_WALKING_ACCEL,
            };
        if !(on_object || collision == CollisionStates::OnTop) {
            current_accel_speed *= PLAYER_AIR_ACCELL_RATIO;
        }

        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            player_acceleration.x += current_accel_speed;
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            player_acceleration.x -= current_accel_speed;
        }

        // configure horizontal acceleration (crude friction)
        let current_friction = f64::min(
            player.velocity.x.abs(),
            match on_object || collision == CollisionStates::OnTop {
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
        // to make the physics continuous and therefore framerate-independent)
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

        // update the position of moving platforms
        for moving_object in &mut moving_objects {
            moving_object.update(frame_time);
        }

        // move with the platform we're stuck to
        if let Some(mut stuck_obj) = stuck_platform {
            if player.collides_with_x(&stuck_obj) {
                stuck_obj.update(frame_time);
                player.center.x += stuck_obj.prev_move.x;
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

        collision = CollisionStates::NoCollision;

        // cache player bounds for physics
        player_bounds = player.bounds();

        if let Some(index) = player.handle_collisions(&moving_objects, &mut collision) {
            stuck_platform = Some(moving_objects[index].clone());
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
                if player_bounds.0 >= bounds.1 - COLLISION_DEPTH_BASE * (1 << j) as f64
                    && player.velocity.x <= 0.0
                {
                    player.center.x = bounds.1 + player.width / 2.0;

                    on_side = true;
                } else if player_bounds.1 <= bounds.0 + COLLISION_DEPTH_BASE * (1 << j) as f64
                    && player.velocity.x >= 0.0
                {
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
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
            || window.is_key_pressed(Key::W, minifb::KeyRepeat::No)
            || window.is_key_pressed(Key::Up, minifb::KeyRepeat::No)
        {
            jump_buffer = JUMP_BUFFER_HUNDRETHSECS;
        }

        if (on_object || collision == CollisionStates::OnTop) && jump_buffer > 0.0 {
            player.velocity.y = JUMP_FORCE;
            jump_buffer = 0.0;
            stuck_platform = None; // if we jump, unstick ourselves
        } else if on_object || collision == CollisionStates::OnTop {
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

        if player.center.x - camera.bottom_left.x < MIN_X_FROM_CAMERA {
            camera.bottom_left.x -= (player.center.x - camera.bottom_left.x - MIN_X_FROM_CAMERA)
                * (player.center.x - camera.bottom_left.x - MIN_X_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - camera.bottom_left.x > MIN_X_FROM_CAMERA {
                camera.bottom_left.x = player.center.x - MIN_X_FROM_CAMERA;
            }
        } else if player.center.x - camera.bottom_left.x > MAX_X_FROM_CAMERA {
            camera.bottom_left.x += (player.center.x - camera.bottom_left.x - MAX_X_FROM_CAMERA)
                * (player.center.x - camera.bottom_left.x - MAX_X_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_X;

            // avoid over-correcting the camera
            if player.center.x - camera.bottom_left.x < MAX_X_FROM_CAMERA {
                camera.bottom_left.x = player.center.x - MAX_X_FROM_CAMERA;
            }
        }

        if player.center.y - camera.bottom_left.y < MIN_Y_FROM_CAMERA {
            camera.bottom_left.y -= (player.center.y - camera.bottom_left.y - MIN_Y_FROM_CAMERA)
                * (player.center.y - camera.bottom_left.y - MIN_Y_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - camera.bottom_left.y > MIN_Y_FROM_CAMERA {
                camera.bottom_left.y = player.center.y - MIN_Y_FROM_CAMERA;
            }
        } else if player.center.y - camera.bottom_left.y > MAX_Y_FROM_CAMERA {
            camera.bottom_left.y += (player.center.y - camera.bottom_left.y - MAX_Y_FROM_CAMERA)
                * (player.center.y - camera.bottom_left.y - MAX_Y_FROM_CAMERA)
                * frame_time
                * CAMERA_MOVING_EASING_Y;

            // avoid over-correcting the camera
            if player.center.y - camera.bottom_left.y < MAX_Y_FROM_CAMERA {
                camera.bottom_left.y = player.center.y - MAX_Y_FROM_CAMERA;
            }
        }

        //
        // graphics
        //

        // put our rendered graphics into our buffer
        camera.render_frame(
            &render_game,   // render function
            &player_bounds, // the player's bounds
            window.is_key_down(Key::LeftShift) || window.is_key_down(Key::RightShift), // if the player is sprinting
            &static_object_bounds, // static object bounds
            &moving_objects
                .iter()
                .map(|object| object.bounds())
                .collect(), // moving object bounds
            &mut window_buffer,    // mutable refrence to our window buffer
        );

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
