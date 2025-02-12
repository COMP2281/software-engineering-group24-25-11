use godot::classes::{
    CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion, XrCamera3D, XrOrigin3D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct PlayerVR {
    base: Base<CharacterBody3D>,
}

const MOVEMENT_SPEED: f64 = 5.;
const GRAVITY: f64 = 9.8;
const JUMP_IMPULSE: f64 = 5.;
const SENSITIVITY: f64 = 0.001;

#[godot_api]
impl ICharacterBody3D for PlayerVR {
    fn init(base: Base<CharacterBody3D>) -> Self {
        // let mut camera = res.base().get_node_as::<XrCamera3D>("XrCamera3D");
        // camera.set_current(true);
        godot_print!("VR Character created...");
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
        if !self.base().is_on_floor() {
            velocity.y = self.base().get_velocity().y - (GRAVITY * delta) as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let ev = event.try_cast::<InputEventMouseMotion>();
        if let Ok(motion) = ev {
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            let mut camera = pivot.get_node_as::<Camera3D>("Camera3D");
            let rel = -motion.get_relative() * SENSITIVITY as f32;
            //let mut pivot_rot = pivot.get_rotation();
            //pivot_rot.y = (pivot_rot.y + rel.x).clamp(0., std::f32::consts::PI);
            //pivot.set_rotation(pivot_rot);
            pivot.rotate_y(rel.x);
            let mut camera_rot = camera.get_rotation();
            camera_rot.x = (camera_rot.x + rel.y)
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
            camera.set_rotation(camera_rot);
            //camera.rotate_x(rel.y);
        }
    }
}
