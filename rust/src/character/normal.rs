use std::f32::consts::PI;

use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,
}

const MOVEMENT_SPEED: f64 = 5.;
const GRAVITY: f64 = 20.;
const JUMP_IMPULSE: f64 = 30.;
const SENSITIVITY: f64 = 0.1;

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        godot_print!("Created the character"); // Prints to the Godot console
        Self { base }
    }
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        // 2D direction input for horizontal movement
        let horizontal_input = input.get_vector(
            "move_left".into(),
            "move_right".into(),
            "move_forward".into(),
            "move_back".into(),
        );
        let pivot = self.base().get_node_as::<Node3D>("Pivot");
        let direction =
            pivot.get_basis() * Vector3::new(horizontal_input.x, 0., horizontal_input.y);

        let mut velocity: Vector3 = direction.normalized_or_zero() * MOVEMENT_SPEED as f32;

        if input.is_action_pressed("jump".into()) && self.base().is_on_floor() {
            velocity.y = JUMP_IMPULSE as f32;
        }
        // TODO: handle outside spaceship - if we even want that
        if !self.base().is_on_floor() {
            velocity.y -= (GRAVITY * delta) as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let ev = event.try_cast::<InputEventMouseMotion>();
        let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
        let mut camera = pivot.get_node_as::<Camera3D>("Camera3D");
        godot_print!("input ev");
        if let Ok(motion) = ev {
            godot_print!("mouse motion");
            let rel = -motion.get_relative() * SENSITIVITY as f32;
            //let rot = pivot.get_rotation();
            let rot = camera.get_rotation();
            godot_print!("rotation {:?}", rot);
            //pivot.set_rotation(Vector3::new(
            //    rot.x,
            //    (rot.y + rel.x * 0.001).clamp(0., PI),
            //    rot.z,
            //));
            pivot.rotate_y(rel.x * 0.01);
            camera.rotate_x(rel.y * 0.01);
            pivot.set_rotation(Vector3::new(
                (rot.x + (rel.y * 0.001)).clamp(0., PI),
                rot.y,
                rot.z,
            ));
        }
    }
}
