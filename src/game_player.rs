use super::{
    camera::Camera,
    constants::{
        BACKGROUND_COLOR, FRICTION_AIR, FRICTION_GROUND, GRAVITY_MOVING_DOWN, GRAVITY_MOVING_UP,
        JUMP_BUFFER_HUNDRETHSECS, JUMP_FORCE, MOVING_OBJECT_COLOR,
        MOVING_PLATFORM_SPEED_JUMP_MODIFIER, NORMAL_PLAYER_COLOR, PLAYER_AIR_ACCELL_RATIO,
        PLAYER_WALKING_ACCEL, STATIC_OBJECT_COLOR, VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT,
        WINDOW_HEIGHT, WINDOW_WIDTH,
    },
    map_loader::Map,
    objects::{bounds_contain_point, CollisionStates, MovingObject, RectObject, Vector2},
};

use minifb::{Key, Window};

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

/**
  plays a game with a supplied map and window
* function will end when the player beats the level or presses escape
* returns if the user tried to terminate the entire program
*/
pub fn play_game(map: &mut Map, window: &mut Window) -> bool {
    // this will be where we write out pixel values
    let mut window_buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // create our player
    let mut player = map.default_player.clone();

    // create our camera
    let mut camera = Camera::new(player.center.x - WINDOW_WIDTH as f64 / 2.0, 0.0);

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

    // this is where we'll store the player's active collision
    let mut collision = CollisionStates::NoCollision;

    // when a player is on a moving platform, we "stick" them to
    // the platform to stop them from bouncing on it as it moves
    let mut stuck_platform: Option<MovingObject> = None;

    while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
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

        // configure horizontal acceleration
        let current_friction = f64::min(
            player.velocity.x.abs(),
            match collision == CollisionStates::OnTop {
                true => FRICTION_GROUND * player.velocity.x.abs(),
                false => FRICTION_AIR * player.velocity.x.abs(),
            },
        );

        // apply friction
        if player.velocity.x < 0.0 {
            player_acceleration_vector.x += current_friction;
        } else {
            player_acceleration_vector.x -= current_friction;
        }

        // move the player (we integrate the player's movement to make
        // the physics continuous and therefore framerate-independent)
        let movement_vector = Vector2::add(
            // accel * t^2 / 2
            &Vector2::multiply(&player_acceleration_vector, frame_time * frame_time / 2.0),
            // vel * t
            &Vector2::multiply(&player.velocity, frame_time),
            // c is already stored in the player's position
            // and will be included when we add this movement
            // vector to the player's current position
        );

        // apply the movement vector we calculated (adds c)
        player.move_by(&movement_vector);

        // update velocity (no integrating is needed as accel * t is exactly the growth in velocity)
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
            // only keep the player stuck if they're still on the platform
            if player.collides_with_x(&stuck_obj) {
                stuck_obj.update(frame_time);
                player.center.x += stuck_obj.prev_move.x;
                // move the player slightly into the platform to keep them stuck
                player.center.y = stuck_obj.bounds().3 + player.height / 2.0 - 0.01;
            }
        }

        stuck_platform = None;

        //
        // collision handling
        //

        // reset collision
        collision = CollisionStates::NoCollision;

        // handle collisions with moving objects, and if we're stuck to
        // an object, update the stuck_platform variable
        if let Some(index) = player.handle_collisions(&map.moving_objects, &mut collision) {
            stuck_platform = Some(map.moving_objects[index].clone());
        }

        // handle collisions with static objects
        player.handle_collisions(&map.static_objects, &mut collision);

        //
        // final physics before rendering graphics
        //

        // decrease our jump buffer
        jump_buffer -= frame_time;

        // reset the player's velocity if they're
        // on the side of an object
        if collision == CollisionStates::OnSide {
            player.velocity.x = 0.0;
        }

        // if any of the space keys are pressed, start jump buffer
        let jump_pressed = window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
            || window.is_key_pressed(Key::W, minifb::KeyRepeat::No)
            || window.is_key_pressed(Key::Up, minifb::KeyRepeat::No);

        if jump_pressed {
            jump_buffer = JUMP_BUFFER_HUNDRETHSECS;
        }

        // handle jumping
        if collision == CollisionStates::OnTop && jump_buffer > 0.0 {
            // if the player is stuck to a platform, add that object's vertical
            // velocity multiplied by a constant to the player's vertical velocity
            let mut additional_y_velocity: f64 = 0.0;
            if let Some(obj) = stuck_platform {
                additional_y_velocity = obj.prev_move.y * MOVING_PLATFORM_SPEED_JUMP_MODIFIER;
            }

            // set the correct vertical velocity
            // and reset the jump buffer
            player.velocity.y = JUMP_FORCE + additional_y_velocity;
            jump_buffer = 0.0;
            stuck_platform = None; // if we jump, unstick ourselves
        }
        // if the player is on the top of or the bottom of an
        // object, reset the player's vertical velocity
        else if collision == CollisionStates::OnTop || collision == CollisionStates::OnBottom {
            player.velocity.y = VERTICAL_VELOCITY_ON_OR_UNDER_OBJECT;
        }

        // cache they player's bounds
        player_bounds = player.bounds();

        // reapawn if the player is too low
        if player.center.y < map.lowest_point {
            player = map.default_player.clone();
        }

        // keep camera centered on player
        camera.keep_centered_on_player(&mut player, frame_time);

        //
        // graphics
        //

        // render our graphics
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

    // update window with the last rendered frame so that
    // any keys pressed last frame don't count as pressed
    // next time they're read with keyrepeat true
    window
        .update_with_buffer(&window_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap_or_else(|error| {
            panic!("Error updating window: {}", error);
        });

    // returns if only esc was pressed to indicate returning to menu
    // or if left ctrl was pressed to indicate closing the entire game
    (window.is_key_down(Key::Escape) && window.is_key_down(Key::LeftCtrl)) || !window.is_open()
}
