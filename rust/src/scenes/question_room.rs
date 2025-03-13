use godot::{
    classes::{CollisionShape3D, MeshInstance3D},
    prelude::*,
};

use crate::question_bank::Question;

use super::question_panel::QuestionPanel;

const PANEL_POSITION: Vector3 = Vector3::new(0., 5., 0.);
pub const ROOM_SIZE: Vector3 = Vector3::new(30.5, 10., 30.5);

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct QuestionRoom {
    pub panel: Option<Gd<QuestionPanel>>,
    base: Base<Node3D>,
}

#[godot_api]
impl QuestionRoom {
    pub fn create(question: Question, time_elapsed: u64) -> Gd<Self> {
        let mut room = load::<PackedScene>(crate::resources::QUESTION_ROOM_SCENE)
            .instantiate_as::<QuestionRoom>();

        let mut panel = QuestionPanel::create_panel(question, time_elapsed);
        panel.set_position(PANEL_POSITION);
        panel.set_name(&GString::from("QuestionPanel"));

        room.add_child(&panel);
        room.bind_mut().panel = Some(panel);

        room
    }
    pub fn open_front_door(&self) {
        let mut door_collsion = self
            .base()
            .get_node_as::<CollisionShape3D>("FrontDoorWall/Door/CollisionShape3D");
        door_collsion.set_disabled(true);
        let mut door_mesh = self
            .base()
            .get_node_as::<MeshInstance3D>("FrontDoorWall/Door/MeshInstance3D");
        door_mesh.hide();
    }

    pub fn open_back_door(&self) {
        let mut door_collsion = self
            .base()
            .get_node_as::<CollisionShape3D>("BackDoorWall/Door/CollisionShape3D");
        door_collsion.set_disabled(true);
        let mut door_mesh = self
            .base()
            .get_node_as::<MeshInstance3D>("BackDoorWall/Door/MeshInstance3D");
        door_mesh.hide();
    }

    pub fn close_back_door(&self) {
        let mut door_collsion = self
            .base()
            .get_node_as::<CollisionShape3D>("BackDoorWall/Door/CollisionShape3D");
        door_collsion.set_disabled(false);
        let mut door_mesh = self
            .base()
            .get_node_as::<MeshInstance3D>("BackDoorWall/Door/MeshInstance3D");
        door_mesh.show();
    }
}
