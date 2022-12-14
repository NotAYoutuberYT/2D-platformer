mod objects;
use objects::{RigidBody, Point, RectObject};

fn main() {
    let object1 = RigidBody {
        center: Point::new(10.0, 10.0),
        width: 5.0,
        height: 5.0,
        density: 1.0,
    };

    let object2 = RigidBody {
        center: Point::new(10.0, 10.0),
        width: 4.0,
        height: 4.0,
        density: 1.0,
    };

    println!("{}", object1.collides_with(&object2));
}
