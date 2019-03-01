extern crate communicator;

mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;
use a::series::of;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    communicator::client::connect();
    of::nested_modules();
    nested_modules();

    let red = Red;
    let yellow = Yellow;
//    let green = TrafficLight::Green;
    let green = Green;
}
