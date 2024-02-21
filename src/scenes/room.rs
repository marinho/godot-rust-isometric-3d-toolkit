use godot::engine::{Node, INode};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Room {
    base: Base<Node>,
}

#[godot_api]
impl Room {
    // #[signal]
    // fn hit();

    // #[func]
    // fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
    // }
}

#[godot_api]
impl INode for Room {
    fn init(base: Base<Node>) -> Self {
        Room {
            base,
        }
    }

    // fn ready(&mut self) {
    // }

    // fn process(&mut self, delta: f64) {
    // }
}