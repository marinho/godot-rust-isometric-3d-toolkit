use godot::engine::{Node, INode};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Level {
    base: Base<Node>,
}

#[godot_api]
impl Level {
    // #[signal]
    // fn hit();

    // #[func]
    // fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
    // }
}

#[godot_api]
impl INode for Level {
    fn init(base: Base<Node>) -> Self {
        Level {
            base,
        }
    }

    // fn ready(&mut self) {
    // }

    // fn process(&mut self, delta: f64) {
    // }
}