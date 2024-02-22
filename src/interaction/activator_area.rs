use godot::engine::{Area3D, IArea3D, Node3D, AudioStream};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct ActivatorArea {
    base: Base<Area3D>,

    #[export_group("Activation")]
    #[export]
    pub activable: Node,

    #[export]
    pub can_activate: bool, // true

    #[export]
    pub can_deactivate: bool, // true

    #[export]
    pub can_input_action: bool, // true

    #[export]
    pub activated_by_player: bool, // true

    #[export]
    pub activated_by_enemy: bool, // false

    #[export]
    pub action_input_balloon: Node, // ActionInputBalloon

    #[export_group("Action")]
    #[export]
    pub text_for_input: GString, // "Input Action"

    #[export]
    pub maximum_input_actions: u16, // 0 for infinite

    #[export]
    pub time_before_next_input_action: f32, // 0.0f

    #[export]
    pub input_name: GString, // "Choose"

    #[export_group("Audio")]
    #[export]
    pub activate_sound: AudioStream, // when player enters the area

    #[export]
    pub deactivate_sound: AudioStream, // when player exits the area

    #[export]
    pub input_action_sound: AudioStream, // when player presses action button

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