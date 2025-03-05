use std::str::FromStr;

use godot::{
    classes::{input::MouseMode, Time},
    prelude::*,
};

use crate::question_bank::{Course, Mode, QuestionBank};

use super::question_panel::QuestionPanel;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct WorldScene {
    pub question_bank: QuestionBank,
    base: Base<Node>,
}

#[godot_api]
impl INode for WorldScene {
    /// this should be avoided, prefer to use create_world_with_bank
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            question_bank: QuestionBank::new(Course::WebDev, Mode::Normal)
                .expect("failed to create question bank"),
        }
    }

    fn ready(&mut self) {
        let scene_tree = self.base().get_tree().expect("world is in scene tree");
        let room = WorldScene::get_room(scene_tree).expect("world has room");
        // Check mode
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);

        let question = self.question_bank.get_question();
        self.question_bank.start_time = Time::singleton().get_ticks_msec();
        QuestionBank::create_panel(question, room, Vector3::new(0., 5., 0.));
    }
}

#[godot_api]
impl WorldScene {
    #[func]
    pub fn create_world_with_bank(course: Course, mode: Mode) -> Gd<Self> {
        let mut world_scene =
            load::<PackedScene>(crate::resources::WORLD_SCENE).instantiate_as::<WorldScene>();
        world_scene.bind_mut().question_bank =
            QuestionBank::new(course, mode).expect("failed to create question bank");
        world_scene
    }

    #[func]
    pub fn answered_correctly(&mut self) {
        let scene_tree = self.base().get_tree().expect("world is in scene tree");
        let mut room = WorldScene::get_room(scene_tree.clone()).expect("world has room");
        let panel = QuestionPanel::find_panel(scene_tree)
            .expect("answered correctly called with a panel existing");
        room.remove_child(&panel);

        self.question_bank.current_difficulty =
            (self.question_bank.current_difficulty + 0.1).clamp(0., 1.);

        let time_taken = Time::singleton()
            .get_ticks_msec()
            .saturating_sub(self.question_bank.start_time);
        self.question_bank.start_time = Time::singleton().get_ticks_msec();
        godot_print!("Took {}ms to answer question", time_taken);

        let question = self.question_bank.get_question();
        QuestionBank::create_panel(question, room, Vector3::new(0., 5., 0.));
    }
    #[func]
    pub fn answered_incorrectly(&mut self) {
        let scene_tree = self.base().get_tree().expect("world is in scene tree");
        let mut room = WorldScene::get_room(scene_tree.clone()).expect("world has room");
        let panel = QuestionPanel::find_panel(scene_tree)
            .expect("answered correctly called with a panel existing");

        let time_taken = Time::singleton()
            .get_ticks_msec()
            .saturating_sub(self.question_bank.start_time);
        self.question_bank.start_time = Time::singleton().get_ticks_msec();
        godot_print!("Took {}ms to answer question", time_taken);

        room.remove_child(&panel);
        self.question_bank.current_difficulty =
            (self.question_bank.current_difficulty - 0.1).clamp(0., 1.);
        let question = self.question_bank.get_question();
        QuestionBank::create_panel(question, room, Vector3::new(0., 5., 0.));
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

    #[func]
    pub fn get_room(scene_tree: Gd<SceneTree>) -> Option<Gd<Node3D>> {
        let path: NodePath =
            NodePath::from_str("/root/Root/World/Room").expect("failed to create room node path");
        scene_tree
            .get_root()
            .unwrap()
            .get_node_or_null(&path)?
            .try_cast::<Node3D>()
            .ok()
    }
}
