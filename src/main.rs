// objects should be used in a way where physics and movement
// is handled, then collisions are done with every object
// in that order every frame
mod objects;
use objects::{MovingObject, MovingObjectDirections, Vector2};

fn main() {
    let mut object1 = MovingObject::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(5.0, -2.0),
        1.0,
        1.0,
        false,
    );

    object1.update(0.5);

    println!(
        "x: {}, y: {}, moving_time: {}, direction: {}",
        object1.center.x,
        object1.center.y,
        object1.moving_time,
        match object1.direction {
            MovingObjectDirections::Leaving => "Leaving",
            MovingObjectDirections::Returning => "Returning",
            MovingObjectDirections::Standstil => "Standstill",
        }
    );
}

// todo: fix easing, make update potentiall swap directions, commit :)
