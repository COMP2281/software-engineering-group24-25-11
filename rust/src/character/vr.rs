use godot::classes::{
    CharacterBody3D, ICharacterBody3D, IXrController3D, RayCast3D, XrCamera3D, XrController3D,
};
use godot::prelude::*;

use crate::scene_manager::SceneManager;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct PlayerVR {
    base: Base<CharacterBody3D>,
}

// FIXME: see if the actions can be listened to with those from the openxr action map through an
// input singleton.

const MOVEMENT_SPEED: f64 = 5.;
const GRAVITY: f64 = 9.8;
const JUMP_IMPULSE: f64 = 5.;
const JOYSTICK_SENSITIVITY: f64 = 0.025;

#[godot_api]
impl PlayerVR {
    #[func]
    fn ray_cast(&mut self) {
        // this raycast is masked to collision layer 8, which only the quiz
        // buttons are attached to. Therefore any collisions means we are
        // looking at a button on the quiz panel.
        let right_controller = self
            .base()
            .get_node_as::<VRController>("XROrigin3D/RightController");
        let ray_cast = right_controller.get_node_as::<RayCast3D>("RayCast3D");
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
                if right_controller.is_button_pressed("trigger") {
                    panel.bind().handle_click();
                };
            }
        }

        if right_controller.is_button_pressed("trigger") {
            world.bind().is_end_button(collider);
        }
    }
}

#[godot_api]
impl ICharacterBody3D for PlayerVR {
    fn physics_process(&mut self, delta: f64) {
        let right = self
            .base()
            .get_node_as::<VRController>("XROrigin3D/RightController");
        let left = self
            .base()
            .get_node_as::<VRController>("XROrigin3D/LeftController");

        let horizontal_input = right.get_vector2("thumbstick");

        let direction = (self.base().get_basis()
            * Vector3::new(horizontal_input.x, 0., -horizontal_input.y))
        .normalized_or_zero();
        // let direction =
        //     (head.get_basis() * Vector3::new(movement.x, 0., -movement.y)).normalized_or_zero();

        let mut velocity: Vector3 = Vector3::new(
            direction.x * MOVEMENT_SPEED as f32,
            0.,
            direction.z * MOVEMENT_SPEED as f32,
        );

        if !self.base().is_on_floor() {
            velocity.y = self.base().get_velocity().y - (GRAVITY * delta) as f32;
        } else if right.is_button_pressed("ax_button") {
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

        // // handle joystick looking
        let rel = -left.get_vector2("thumbstick") * JOYSTICK_SENSITIVITY as f32;

        let mut rot = self.base().get_rotation();
        rot.y = (rot.y + rel.x) % (2. * std::f32::consts::PI);
        self.base_mut().set_rotation(rot);

        // let mut head = self.base().get_node_as::<Node3D>("Head");
        // let mut head_rot = head.get_rotation();
        // head_rot.x =
        //     (head_rot.x + (rel.y)).clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
        // head.set_rotation(head_rot);

        self.ray_cast();
    }
    fn ready(&mut self) {
        let mut camera = self
            .base()
            .get_node_as::<XrCamera3D>("XROrigin3D/XRCamera3D");
        camera.set_current(true);
    }
}

#[derive(GodotClass)]
#[class(init, base=XrController3D)]
pub struct VRController {
    base: Base<XrController3D>,
}

#[godot_api]
impl IXrController3D for VRController {
    fn ready(&mut self) {
        let primary = self.base().callable("primary");
        self.base_mut().connect("primary", &primary);
    }
}
#[godot_api]
impl VRController {
    #[func]
    fn primary(&mut self, out: Vector2) {
        godot_print!("received event: {out:?}");
    }
}
