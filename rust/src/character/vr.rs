use godot::classes::{CharacterBody3D, ICharacterBody3D, RayCast3D, XrCamera3D, XrController3D};
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
        let ray_cast = self
            .base()
            .get_node_as::<RayCast3D>("XROrigin3D/RightController/RayCast3D");
        let Some(mut world) = SceneManager::get_manager(self.base().clone().upcast())
            .bind()
            .get_world_scene()
        else {
            return;
        };
        let Some(collider) = ray_cast.get_collider() else {
            return;
        };

        let right = self
            .base()
            .get_node_as::<VRController>("XROrigin3D/RightController");

        let world_bind = world.bind_mut();
        let rooms = &world_bind.rooms.duplicate_shallow();
        drop(world_bind);

        for mut room in rooms.iter_shared() {
            if let Some(ref mut panel) = room.bind_mut().panel {
                panel.bind_mut().set_looking_at(collider.clone());
                if right.is_button_pressed("trigger") {
                    panel.bind_mut().handle_click();
                };
            }
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

        let movement = right.get_vector2("thumbstick");
        let head = self
            .base()
            .get_node_as::<XrCamera3D>("XROrigin3D/XRCamera3D");
        let direction =
            (head.get_basis() * Vector3::new(movement.x, 0., -movement.y)).normalized_or_zero();

        let mut velocity: Vector3 = Vector3::new(
            direction.x * MOVEMENT_SPEED as f32,
            0.,
            direction.z * MOVEMENT_SPEED as f32,
        );

        if !self.base().is_on_floor() {
            velocity.y = self.base().get_velocity().y - (GRAVITY * delta) as f32;
        } else if right.is_button_pressed(&StringName::from("a click")) {
            velocity.y = JUMP_IMPULSE as f32;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        // // handle joystick looking
        let rel = -left.get_vector2("thumbstick") * JOYSTICK_SENSITIVITY as f32;
        let mut head_rot = self.base().get_rotation();
        head_rot.y += rel.x;
        self.base_mut().set_rotation(head_rot);
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
