use godot::engine::{Area3D, IArea3D, Node3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct ActivatorArea {
    base: Base<Area3D>,
}

#[godot_api]
impl ActivatorArea {
    #[signal]
    fn activate();

    #[signal]
    fn activate_with_body(&self, body: &Node3D);

    #[signal]
    fn deactivate();

    #[signal]
    fn deactivate_with_body(&self, body: &Node3D);

    #[signal]
    fn input_action();

    // #[func]
    // fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
    // }
}

#[godot_api]
impl IArea3D for ActivatorArea {
    fn init(base: Base<Area3D>) -> Self {
        ActivatorArea {
            base,
        }
    }

    // fn ready(&mut self) {
    // }

    // fn process(&mut self, delta: f64) {
    // }
}