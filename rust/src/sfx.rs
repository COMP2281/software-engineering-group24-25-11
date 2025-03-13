use godot::{classes::AudioStreamPlayer3D, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SFXController {
    base: Base<Node>,
}

#[godot_api]
impl SFXController {
    #[func]
    pub fn play_menu_select(&self) {
        let mut audio = self.base().get_node_as::<AudioStreamPlayer>("MenuSelect");
        audio.play();
    }

    #[func]
    pub fn play_menu_hover(&self) {
        let mut audio = self.base().get_node_as::<AudioStreamPlayer>("MenuHover");
        audio.play();
    }

    #[func]
    pub fn play_qp_hover(&self, position: Vector3) {
        let mut audio = self
            .base()
            .get_node_as::<AudioStreamPlayer3D>("QuestionPanelHover");
        audio.set_position(position);
        audio.play();
    }
    #[func]
    pub fn play_qp_select(&self, position: Vector3) {
        let mut audio = self
            .base()
            .get_node_as::<AudioStreamPlayer3D>("QuestionPanelSelect");
        audio.set_position(position);
        audio.play();
    }
    #[func]
    pub fn play_qp_correct(&self, position: Vector3) {
        let mut audio = self
            .base()
            .get_node_as::<AudioStreamPlayer3D>("QuestionPanelCorrect");
        audio.set_position(position);
        audio.play();
    }
    #[func]
    pub fn play_qp_incorrect(&self, position: Vector3) {
        let mut audio = self
            .base()
            .get_node_as::<AudioStreamPlayer3D>("QuestionPanelIncorrect");
        audio.set_position(position);
        audio.play();
    }

    #[func]
    pub fn start_walking(&self, position: Vector3) {
        let mut audio = self.base().get_node_as::<AudioStreamPlayer3D>("Walking");
        audio.set_position(position);
        if !audio.is_playing() {
            audio.play();
        }
    }

    #[func]
    pub fn stop_walking(&self) {
        let mut audio = self.base().get_node_as::<AudioStreamPlayer3D>("Walking");
        audio.stop();
    }
}
