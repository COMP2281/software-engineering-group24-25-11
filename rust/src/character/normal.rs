use cfg_if::cfg_if;

#[cfg(target_arch = "wasm32")]
use godot::classes::{input::MouseMode, InputEventMouseButton};
use godot::classes::{
    CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion, RayCast3D,
};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::scene_manager::SceneManager;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player3D {
    base: Base<CharacterBody3D>,
}

const MOVEMENT_SPEED: f64 = 5.;
const GRAVITY: f64 = 9.8;
const JUMP_IMPULSE: f64 = 5.;
const SENSITIVITY: f64 = 0.001;
const JOYSTICK_SENSITIVITY: f64 = 0.025;

#[godot_api]
impl Player3D {
    #[func]
    fn ray_cast(&mut self) {
        // this raycast is masked to collision layer 8, which only the quiz
        // buttons are attached to. Therefore any collisions means we are
        // looking at a button on the quiz panel.

        let ray_cast = self.base().get_node_as::<RayCast3D>("Head/RayCast3D");
        let input = Input::singleton();
        let Some(world) = SceneManager::get_manager(self.base().clone().upcast())
            .bind()
            .get_world_scene()
        else {
            godot_warn!("failed to find world");
            return;
        };

        let rooms = &world.bind().rooms.duplicate_shallow();

        let Some(collider) = ray_cast.get_collider() else {
            for room in rooms.iter_shared() {
                if let Some(ref panel) = room.bind().panel {
                    panel.bind().clear_looking_at();
                }
            }
            return;
        };

        for room in rooms.iter_shared() {
            let room = room.bind();
            if let Some(ref panel) = room.panel {
                panel.bind().set_looking_at(collider.clone());
                if input.is_action_just_pressed("interact") {
                    panel.bind().handle_click();
                };
            }
        }

        if input.is_action_just_pressed("interact") {
            world.bind().is_end_button(collider);
        }
    }
}

#[godot_api]
impl ICharacterBody3D for Player3D {
    fn init(base: Base<CharacterBody3D>) -> Self {
        godot_print!("Character created...");
        Self { base }
    }
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        // 2D direction input for horizontal movement
        let horizontal_input =
            input.get_vector("move_left", "move_right", "move_forward", "move_back");
        let direction = (self.base().get_basis()
            * Vector3::new(horizontal_input.x, 0., horizontal_input.y))
        .normalized_or_zero();

        let mut velocity: Vector3 = Vector3::new(
            direction.x * MOVEMENT_SPEED as f32,
            0.,
            direction.z * MOVEMENT_SPEED as f32,
        );

        if !self.base().is_on_floor() {
            velocity.y = self.base().get_velocity().y - (GRAVITY * delta) as f32;
        } else if input.is_action_pressed(&StringName::from("jump")) {
            velocity.y = JUMP_IMPULSE as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let sfx = scene_manager.bind().get_sfx_controller();
        // FIXME: remove walking sound when ended
        if velocity.is_zero_approx() {
            sfx.bind().stop_walking()
        } else {
            sfx.bind().start_walking(self.base().get_global_position())
        }

        // handle joystick looking
        let rel = input.get_vector("look_left", "look_right", "look_forward", "look_back");
        let rel = -rel * JOYSTICK_SENSITIVITY as f32;

        let mut rot = self.base().get_rotation();
        rot.y = (rot.y + rel.x) % (2. * std::f32::consts::PI);
        self.base_mut().set_rotation(rot);

        let mut head = self.base().get_node_as::<Node3D>("Head");
        let mut head_rot = head.get_rotation();
        head_rot.x =
            (head_rot.x + (rel.y)).clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
        head.set_rotation(head_rot);
    }
    fn ready(&mut self) {
        let mut camera = self.base().get_node_as::<Camera3D>("Head/Camera3D");
        camera.set_current(true);
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        // In the web, we want to recapture the mouse if it has been uncaptured,
        // but only when the user clicks back into the game. Likewise, we should
        // also only respond to mouse events if the mouse is captured.
        let input = Input::singleton();
        cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                if input.get_mouse_mode() != MouseMode::CAPTURED {
                    if event.clone().try_cast::<InputEventMouseButton>().is_ok() {
                        godot_print!("setting mouse mode to captured");
                        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
                    }
                    if input.is_action_just_pressed("menu_button") {
                        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
                        scene_manager.bind().pause_game();
                    }
                    return;
                };
            }
        };

        let ev = event.try_cast::<InputEventMouseMotion>();
        if let Ok(motion) = ev {
            // INFO: Firefox mouse capture does not keep mouse locked in center, and dynamically
            // re-centres the mouse after input, sending another mouse motion event, skewing the
            // mouse movement on firefox.
            let rel = -motion.get_screen_relative() * SENSITIVITY as f32;

            let mut rot = self.base().get_rotation();
            rot.y = (rot.y + rel.x) % (2. * std::f32::consts::PI);
            self.base_mut().set_rotation(rot);

            // Rotate around local X (pitch), but clamp to avoid flipping
            let mut head = self.base().get_node_as::<Node3D>("Head");
            let mut head_rot = head.get_rotation();
            head_rot.x = (head_rot.x + (rel.y))
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
            head.set_rotation(head_rot);
        }
        self.ray_cast();

        if input.is_action_just_pressed("menu_button") {
            let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
            scene_manager.bind().pause_game();
            return;
        }
    }
}
