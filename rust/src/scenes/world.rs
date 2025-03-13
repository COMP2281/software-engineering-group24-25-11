use std::str::FromStr;
use std::time::Duration;

use godot::classes::tween::{EaseType, TransitionType};
use godot::classes::{Label, ProgressBar, Time};
use godot::{classes::input::MouseMode, obj::WithBaseField, prelude::*};

use crate::question_bank::{AnsweredQuestion, Course, Mode, QuestionBank};

use crate::scene_manager::SceneManager;
use crate::scenes::question_room::QuestionRoom;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
/// Manual initialisation should be avoided in favour of create_world_with_bank
pub struct WorldScene {
    pub time_elapsed: u64,
    pub previous_time: u64,

    pub question_bank: QuestionBank,
    pub rooms: Array<Gd<QuestionRoom>>,
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for WorldScene {
    fn ready(&mut self) {
        godot_print!("setting mouse to captured");
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }

    fn process(&mut self, _delta: f64) {
        let current = Time::singleton().get_ticks_msec();
        self.time_elapsed += current - self.previous_time;
        self.previous_time = current;

        let mut time = self
            .base()
            .get_node_as::<Label>("Control/VBoxContainer/HBoxContainer/Time");
        time.set_text(&format!(
            "{}",
            humantime::format_duration(Duration::from_millis(self.time_elapsed)),
        ));
    }
}

#[godot_api]
impl WorldScene {
    #[func]
    pub fn create_world_with_bank(course: Course, mode: Mode) -> Gd<Self> {
        godot_print!("creating world with bank: {course:?}");
        let mut world_scene =
            load::<PackedScene>(crate::resources::WORLD_SCENE).instantiate_as::<WorldScene>();
        world_scene.set_name("World");

        let mut start_room =
            load::<PackedScene>(crate::resources::START_ROOM_SCENE).instantiate_as::<Node3D>();
        start_room.set_name("StartRoom");
        world_scene.add_child(&start_room);

        let mut question_bank =
            QuestionBank::new(course, mode, Some(3), world_scene.bind().time_elapsed)
                .expect("failed to create question bank");

        let first_question = question_bank
            .get_question()
            .expect("new world has atleast one question remaining");

        world_scene.bind_mut().question_bank = question_bank;

        let mut room = QuestionRoom::create(first_question, world_scene.bind().time_elapsed);
        world_scene.add_child(&room);
        room.bind().open_back_door();
        room.set_position(Vector3::new(0., 0., -30.));
        world_scene.bind_mut().rooms.push(&room);

        godot_print!("finished creating world");
        world_scene
    }

    pub fn question_answered(&mut self, question: AnsweredQuestion) {
        self.question_bank.submit_answer(question);

        if self.rooms.len() >= 3 {
            let oldest_room = self
                .rooms
                .pop_front()
                .expect("atleast one room when question answered question");
            self.base_mut().remove_child(&oldest_room);
            self.rooms
                .front()
                .expect("atleast one room when question")
                .bind()
                .close_back_door();
        }

        let current_room = self
            .rooms
            .back()
            .expect("atleast one room when question answered correctly");
        current_room.bind().open_front_door();

        if let Some(question) = self.question_bank.get_question() {
            let mut room = QuestionRoom::create(question, self.time_elapsed);
            room.bind_mut().open_back_door();
            let position = current_room.get_position()
                - Vector3::new(0., 0., crate::scenes::question_room::ROOM_SIZE.z);
            room.set_position(position);
            self.rooms.push(&room);
            self.base_mut().add_child(&room);
        } else {
            let mut end_room =
                load::<PackedScene>(crate::resources::END_ROOM_SCENE).instantiate_as::<Node3D>();
            end_room.set_name("EndRoom");
            let position = current_room.get_position()
                - Vector3::new(0., 0., crate::scenes::question_room::ROOM_SIZE.z);
            end_room.set_position(position);
            self.base_mut().add_child(&end_room);
        };

        // Update HUD
        let progress = (self.question_bank.answered_questions.len() as f64
            / self.question_bank.question_limit as f64)
            * 100.;
        let mut bar = self
            .base()
            .get_node_as::<ProgressBar>("Control/VBoxContainer/ProgressBar");
        bar.create_tween()
            .expect("failed to create tween")
            .set_ease(EaseType::OUT)
            .expect("failed to set tween easing")
            .set_trans(TransitionType::EXPO)
            .expect("failed to set tween transition type")
            .tween_property(&bar, "value", &progress.to_variant(), 1.0);
        let mut completed = self
            .base()
            .get_node_as::<Label>("Control/VBoxContainer/HBoxContainer/Completed");
        completed.set_text(&format!(
            "{}/{}",
            self.question_bank.answered_questions.len(),
            self.question_bank.question_limit
        ));
        let mut points = self
            .base()
            .get_node_as::<Label>("Control/VBoxContainer/HBoxContainer/Points");
        points.set_text(&format!("{}pts", self.question_bank.score));
    }

    #[func]
    pub fn is_end_button(&self, obj: Gd<Object>) {
        if let Some(end_button) = self
            .base()
            .try_get_node_as::<Node3D>("EndRoom/Display/EndButton")
        {
            if obj == end_button.upcast() {
                let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
                scene_manager
                    .bind()
                    .show_completion_screen(self.question_bank.clone(), self.time_elapsed);
                godot_print!("end button clicked")
            };
        }
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
