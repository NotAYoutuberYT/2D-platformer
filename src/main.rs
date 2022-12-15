mod objects;

use objects::*;

fn main() {
    let mut object = MovingObject::new(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), 3.0, 1.0, false);

    object.update(1.2);

    println!("x: {}, y: {}, direction: {}", object.center.x, object.center.y, match object.direction {
        MovingObjectDirections::Leaving => "Leaving",
        MovingObjectDirections::Returning => "Returning",
        _ => "Standstill",
    });
}