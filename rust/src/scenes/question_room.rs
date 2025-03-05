use godot::prelude::*;

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
    pub fn create(question: Question) -> Gd<Self> {
        let mut room = load::<PackedScene>(crate::resources::QUESTION_ROOM_SCENE)
            .instantiate_as::<QuestionRoom>();

        let mut panel = QuestionPanel::create_panel(question);
        panel.set_position(PANEL_POSITION);
        panel.set_name(&GString::from("QuestionPanel"));

        room.add_child(&panel);
        room.bind_mut().panel = Some(panel);

        room
    }
}
