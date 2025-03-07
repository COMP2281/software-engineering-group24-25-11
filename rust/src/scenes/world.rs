use std::str::FromStr;

use godot::{classes::input::MouseMode, obj::WithBaseField, prelude::*};

use crate::question_bank::{AnsweredQuestion, Course, Mode, QuestionBank};

use crate::scenes::question_room::QuestionRoom;

#[derive(GodotClass)]
#[class(init, base=Node)]
/// Manual initialisation should be avoided in favour of create_world_with_bank
pub struct WorldScene {
    pub question_bank: QuestionBank,
    pub rooms: Array<Gd<QuestionRoom>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for WorldScene {
    fn ready(&mut self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }
}

#[godot_api]
impl WorldScene {
    #[func]
    pub fn create_world_with_bank(course: Course, mode: Mode) -> Gd<Self> {
        let mut world_scene =
            load::<PackedScene>(crate::resources::WORLD_SCENE).instantiate_as::<WorldScene>();
        world_scene.set_name("World");

        let mut question_bank =
            QuestionBank::new(course, mode, Some(15)).expect("failed to create question bank");

        let first_question = question_bank
            .get_question()
            .expect("new world has atleast one question remaining");

        world_scene.bind_mut().question_bank = question_bank;

        let mut room = QuestionRoom::create(first_question);
        room.set_position(Vector3::new(0., 0., -10.));
        world_scene.add_child(&room);
        world_scene.bind_mut().rooms.push(&room);

        world_scene
    }

    pub fn question_answered(&mut self, question: AnsweredQuestion) {
        if self.rooms.len() >= 3 {
            let oldest_room = self
                .rooms
                .pop_front()
                .expect("atleast one room when question answered correctly");
            self.base_mut().remove_child(&oldest_room);
        }

        let prev_room = self
            .rooms
            .back()
            .expect("atleast one room when question answered correctly");

        self.question_bank
            .adjust_difficulty(if question.correct { 0.1 } else { -0.1 });

        if let Some(question) = self.question_bank.get_question() {
            let mut room = QuestionRoom::create(question);
            room.set_position(
                prev_room.get_position()
                    - Vector3::new(0., 0., crate::scenes::question_room::ROOM_SIZE.z),
            );
            self.rooms.push(&room);
            self.base_mut().add_child(&room);
        } else {
        };
    }

    #[func]
    pub fn get_world(scene_tree: Gd<SceneTree>) -> Option<Gd<Self>> {
        let path: NodePath =
            NodePath::from_str("/root/Root/World").expect("failed to create world node path");
        scene_tree
            .get_root()
            .unwrap()
            .get_node_or_null(&path)?
            .try_cast::<Self>()
            .ok()
    }
}
