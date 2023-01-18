/* This project uses a library (known as a crate in Rust) called minifb. It is
owned by Daniel Collin. It can be found on crates.io at https://crates.io/crates/minifb
or on GitHub at https://github.com/emoon/rust_minifb. To use in a project, put the line
of code "minifb = "0.23" into the cargo.toml file of a cargo-initilized project. This
crate is what allows me to open a window and write rgb values to each pixel of the window. All
code for representing objects, rendring those objects, and performing physics is written by Bryce Holland. */
extern crate minifb;
use std::io::Write;

use minifb::{Key, Window, WindowOptions};

mod objects;
use objects::{
    bounds_contain_point, CollisionStates, MovingObject, RectObject, RigidBody, Vector2,
};

mod camera;
use camera::Camera;

mod map_loader;
use map_loader::Map;

mod constants;
use constants::*; // all my constants have very specific names, so I'm comfortable doing this

// this is the function we use to render the game
fn render_game(
    world_point: Vector2,
    player_bounds: &(f64, f64, f64, f64),
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

    if player_collision {
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
        center: Vector2::new(0.0, 0.0),
        width: 20.0,
        height: 40.0,

        velocity: Vector2::new(0.0, 0.0),
        static_friction: false,
    };

    let mut map: Map = Map::new();

    // loads map user wants to play
    std::print!("Input level you want to play: ");
    std::io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read stdin");
    map.load_map(input.trim().parse().expect("failed to parse input as u32"));

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
    let static_object_bounds: Vec<(f64, f64, f64, f64)> = map
        .static_objects
        .iter()
        .map(|object| object.bounds())
        .collect();
    let mut player_bounds: (f64, f64, f64, f64);

    // this prevent "bouncing" on downward moving platforms
    let mut stuck_platform: Option<MovingObject> = None;

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
        let mut player_acceleration_vector: Vector2 = Vector2::new(0.0, 0.0);

        // configure vertical acceleration (gravity)
        player_acceleration_vector.y = match player.velocity.y <= 0.0 {
            false => GRAVITY_MOVING_UP,
            true => GRAVITY_MOVING_DOWN,
        };

        // configure horizontal acceleration (movement)
        let mut current_x_accell = PLAYER_WALKING_ACCEL;
        if collision != CollisionStates::OnTop {
            current_x_accell *= PLAYER_AIR_ACCELL_RATIO;
        }

        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            player_acceleration_vector.x += current_x_accell;
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            player_acceleration_vector.x -= current_x_accell;
        }

        // configure horizontal acceleration (crude friction)
        let current_friction = f64::min(
            player.velocity.x.abs(),
            match collision == CollisionStates::OnTop {
                true => FRICTION_GROUND * player.velocity.x.abs(),
                false => FRICTION_AIR * player.velocity.x.abs(),
            },
        );

        if player.velocity.x < 0.0 {
            player_acceleration_vector.x += current_friction;
        } else {
            player_acceleration_vector.x -= current_friction;
        }

        // move the player (we integrate the player's movement instead of approximating
        // to make the physics continuous and therefore framerate-independent)
        let movement_vector: Vector2 = Vector2::add(
            &Vector2::multiply(&player_acceleration_vector, frame_time * frame_time / 2.0),
            &Vector2::multiply(&player.velocity, frame_time),
        );
        player.move_by(&movement_vector);

        // update velocity
        Vector2::multiply(&player_acceleration_vector, frame_time).add_to(&mut player.velocity);

        //
        // moving platform stuff
        //

        // update the position of moving platforms
        for moving_object in &mut map.moving_objects {
            moving_object.update(frame_time);
        }

        // move with the platform we're stuck to
        if let Some(mut stuck_obj) = stuck_platform {
            if player.collides_with_x(&stuck_obj) {
                stuck_obj.update(frame_time);
                player.center.x += stuck_obj.prev_move.x;
                player.center.y = stuck_obj.bounds().3 + player.height / 2.0 - 0.01;
                // move the player slightly into the platform
            }
        }

        stuck_platform = None;

        //
        // collision handling
        //

        collision = CollisionStates::NoCollision;

        if let Some(index) = player.handle_collisions(&map.moving_objects, &mut collision) {
            stuck_platform = Some(map.moving_objects[index].clone());
        }

        player.handle_collisions(&map.static_objects, &mut collision);

        //
        // final physics before rendering graphics
        //

        jump_buffer -= frame_time;

        if collision == CollisionStates::OnSide {
            player.velocity.x = 0.0;
        }

        // if space is pressed, start jump buffer
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
            || window.is_key_pressed(Key::W, minifb::KeyRepeat::No)
            || window.is_key_pressed(Key::Up, minifb::KeyRepeat::No)
        {
            jump_buffer = JUMP_BUFFER_HUNDRETHSECS;
        }

        if collision == CollisionStates::OnTop && jump_buffer > 0.0 {
            // adding some of the platform's velocity to the player's
            // allows for satisfying jumps
            let mut additional_y_velocity: f64 = 0.0;
            if let Some(obj) = stuck_platform {
                additional_y_velocity = obj.prev_move.y * MOVING_PLATFORM_SPEED_JUMP_MODIFIER;
            }

            player.velocity.y = JUMP_FORCE + additional_y_velocity;
            jump_buffer = 0.0;
            stuck_platform = None; // if we jump, unstick ourselves
        } else if collision == CollisionStates::OnTop {
            player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        } else if collision == CollisionStates::OnBottom {
            player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        }

        // recache bounds for graphics
        player_bounds = player.bounds();

        // respawn (temporary for prototype)
        if player.center.y < -120.0 {
            player.center = Vector2::new(0.0, 30.0);
        }

        // keep camera centered on player
        camera.keep_centered_on_player(&mut player, frame_time);

        //
        // graphics
        //

        // put our rendered graphics into our buffer
        camera.render_frame(
            &render_game,          // render function
            &player_bounds,        // the player's bounds
            &static_object_bounds, // static object bounds
            &map.moving_objects
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
