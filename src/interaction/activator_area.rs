use godot::engine::{Area3D, AudioStream, IArea3D, PhysicsBody3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct ActivatorArea {
    base: Base<Area3D>,

    // #[export_group("Activation")]
    #[export]
    pub activable: Option<Gd<Node>>,

    #[export]
    can_activate: bool, // true

    #[export]
    can_deactivate: bool, // true

    #[export]
    can_input_action: bool, // true

    #[export]
    activated_by_player: bool, // true

    #[export]
    activated_by_enemy: bool, // false

    #[export]
    action_input_balloon: Option<Gd<Node>>, // ActionInputBalloon

    // #[export_group("Action")]
    #[export]
    text_for_input: GString, // "Input Action"

    #[export]
    maximum_input_actions: u16, // 0 for infinite

    #[export]
    time_before_next_input_action: f32, // 0.0f

    #[export]
    input_name: GString, // "Choose"

    // #[export_group("Audio")]
    #[export]
    activate_sound: Option<Gd<AudioStream>>, // when player enters the area

    #[export]
    deactivate_sound: Option<Gd<AudioStream>>, // when player exits the area

    #[export]
    input_action_sound: Option<Gd<AudioStream>>, // when player presses action button

}

#[godot_api]
impl ActivatorArea {
    #[signal]
    fn activate();

    #[signal]
    fn activate_with_body(body: Gd<PhysicsBody3D>);

    #[signal]
    fn deactivate();

    #[signal]
    fn deactivate_with_body(body: Gd<PhysicsBody3D>);

    #[signal]
    fn input_action();

    // #[func]
    // fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
    // }
}

#[godot_api]
impl IArea3D for ActivatorArea {
    fn init(base: Base<Area3D>) -> Self {
        Self {
            base,
            activable: None,
            can_activate: true,
            can_deactivate: true,
            can_input_action: true,
            activated_by_player: true,
            activated_by_enemy: false,
            text_for_input: GString::from("Input Action"),
            action_input_balloon: None,
            maximum_input_actions: 0,
            time_before_next_input_action: 0.0,
            input_name: GString::from("Choose"),
            activate_sound: None,
            deactivate_sound: None,
            input_action_sound: None,
        }
    }

    // fn ready(&mut self) {
    // }

    // fn process(&mut self, delta: f64) {
    //     let input = Input::singleton();
    //     if Input::is_action_pressed(&input, StringName::from_str(self.input_name.to_string())) {
    //         println!("Input action pressed"); // XXX
    //         // self.emit_signal("input_action", &[]);
    //     }
    // }
}