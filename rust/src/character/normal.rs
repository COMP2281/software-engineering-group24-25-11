use cfg_if::cfg_if;
use godot::classes::input::MouseMode;
use godot::classes::{
    CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseButton, InputEventMouseMotion,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player3D {
    base: Base<CharacterBody3D>,
}

const MOVEMENT_SPEED: f64 = 5.;
const GRAVITY: f64 = 9.8;
const JUMP_IMPULSE: f64 = 5.;
const SENSITIVITY: f64 = 0.001;

#[godot_api]
impl ICharacterBody3D for Player3D {
    fn init(base: Base<CharacterBody3D>) -> Self {
        godot_print!("Character created...");
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
        let camera = self.base().get_node_as::<Camera3D>("Camera3D");
        let direction = (camera.get_basis()
            * Vector3::new(horizontal_input.x, 0., horizontal_input.y))
        .normalized_or_zero();

        let mut velocity: Vector3 = Vector3::new(
            direction.x * MOVEMENT_SPEED as f32,
            0.,
            direction.z * MOVEMENT_SPEED as f32,
        );

        if !self.base().is_on_floor() {
            velocity.y = self.base().get_velocity().y - (GRAVITY * delta) as f32;
        } else if input.is_action_pressed("jump".into()) {
            velocity.y = JUMP_IMPULSE as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
    fn ready(&mut self) {
        let mut camera = self.base().get_node_as::<Camera3D>("Camera3D");
        camera.set_current(true);
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        // In the web, we want to recapture the mouse if it has been uncaptured,
        // but only when the user clicks back into the game. Likewise, we should
        // also only respond to mouse events if the mouse is captured.
        cfg_if::cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                let mut input = Input::singleton();
                if input.get_mouse_mode() != MouseMode::CAPTURED {
                    if event.clone().try_cast::<InputEventMouseButton>().is_ok() {
                        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
                    }
                    return;
                };
            }
        };
        let ev = event.try_cast::<InputEventMouseMotion>();
        if let Ok(motion) = ev {
            let mut camera = self.base().get_node_as::<Camera3D>("Camera3D");
            let rel = -motion.get_relative() * SENSITIVITY as f32;
            let mut camera_rot = camera.get_rotation();
            camera_rot.y += rel.x;
            camera_rot.x = (camera_rot.x + rel.y)
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
            camera.set_rotation(camera_rot);
        }
    }
}
