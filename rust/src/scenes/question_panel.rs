use std::{
    collections::VecDeque,
    f32::consts::PI,
    sync::{Arc, Mutex, RwLock},
};

use godot::{
    classes::{
        label_3d::DrawFlags, text_server::AutowrapMode, AnimatableBody3D, BoxMesh, BoxShape3D,
        CollisionShape3D, Label3D, MeshInstance3D, StandardMaterial3D, StaticBody3D,
    },
    obj::WithBaseField,
    prelude::*,
};

use crate::{question_bank::Question, resources::materials, scene_manager::SceneManager};

#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct QuestionPanel {
    question: Question,
    start_time_ms: u64,

    currently_selected: Arc<RwLock<VecDeque<i64>>>,
    /// 0..n-1 are the choices buttons, whereas the last object is the submit button.
    button_objects: Vec<Gd<Object>>,
    // index of the button currently being looked at
    looking_at: Arc<Mutex<Option<i64>>>,
    // whether the panel has already been submitted, if it has no actions are allowed on it.
    submitted: Arc<Mutex<bool>>,
    base: Base<StaticBody3D>,
}

const BUTTON_SIZE: Vector3 = Vector3::new(8.9, 0.5, 0.4);
const BUTTON_SPACER: f32 = 0.1;
const PANEL_SIZE: Vector3 = Vector3::new(10., BUTTON_SIZE.y * 12., 0.2);
// by default, one pixel = 0.005m
const TEXT_WIDTH_PX: f32 = 8.5 / 0.005;

const SUBMIT_TEXT: &str = "Submit";

#[godot_api]
impl QuestionPanel {
    pub fn create_panel(question: Question, time_elapsed: u64) -> Gd<Self> {
        let mut panel = Gd::from_init_fn(|base| Self {
            question,
            start_time_ms: time_elapsed,

            currently_selected: Arc::new(RwLock::new(VecDeque::new())),
            button_objects: Vec::new(),
            looking_at: Arc::new(Mutex::new(None)),
            submitted: Arc::new(Mutex::new(false)),

            base,
        });

        panel.bind_mut().add_body();
        panel.bind_mut().add_question();
        panel.bind_mut().add_choices();
        panel.bind_mut().add_submit_button();

        panel
    }

    #[func]
    fn add_body(&mut self) {
        // NOTE: must be added to the scene otherwise it will cause memory leaks.
        let mut panel_mesh = MeshInstance3D::new_alloc();
        let mut panel_collision = CollisionShape3D::new_alloc();

        let background = load::<StandardMaterial3D>(materials::QP_BACKGROUND);

        let mut panel_mesh_inner = BoxMesh::new_gd();
        panel_mesh_inner.set_size(PANEL_SIZE);
        panel_mesh_inner.set_material(&background);
        panel_mesh.set_mesh(&panel_mesh_inner);

        let mut panel_collision_inner = BoxShape3D::new_gd();
        panel_collision_inner.set_size(PANEL_SIZE);
        panel_collision.set_shape(&panel_collision_inner);

        self.base_mut().add_child(&panel_mesh);
        self.base_mut().add_child(&panel_collision);
    }

    #[func]
    fn add_question(&mut self) {
        for i in [(-PANEL_SIZE.z / 2.) - 0.001, (PANEL_SIZE.z / 2.) + 0.001].iter() {
            let mut question = Self::new_label();
            question.set_text(&self.question.question);
            question.set_position(Vector3::new(0., 3.5 * BUTTON_SIZE.y, *i));
            if i.is_sign_negative() {
                question.set_rotation(Vector3::new(0., PI, 0.));
            };
            self.base_mut().add_child(&question);
        }
    }

    #[func]
    fn add_choices(&mut self) {
        let normal = load::<StandardMaterial3D>(materials::QP_NORMAL);
        for (idx, choice) in self.question.choices.clone().iter().enumerate() {
            let mut btn = self.create_button(normal.clone(), GString::from(choice), BUTTON_SIZE);
            btn.set_position(Vector3::new(
                0.,
                (PANEL_SIZE.y / 2.) - 2.5 - ((BUTTON_SIZE.y + BUTTON_SPACER) * idx as f32),
                0.,
            ));

            self.button_objects.push(btn.clone().upcast());
            self.base_mut().add_child(&btn);
        }
    }
    #[func]
    fn add_submit_button(&mut self) {
        let submit = load::<StandardMaterial3D>(materials::QP_SUBMIT);
        let mut btn = self.create_button(submit, SUBMIT_TEXT.into(), BUTTON_SIZE);

        btn.set_position(Vector3::new(0., -(PANEL_SIZE.y / 2.) + 1., 0.));

        self.button_objects.push(btn.clone().upcast());
        self.base_mut().add_child(&btn);
    }

    #[func]
    /// NOTE: button created must be added to the tree otherwise it will be unmanaged memory.
    fn create_button(
        &mut self,
        material: Gd<StandardMaterial3D>,
        text: GString,
        size: Vector3,
    ) -> Gd<AnimatableBody3D> {
        let mut body = AnimatableBody3D::new_alloc();
        let mut button_mesh = MeshInstance3D::new_alloc();
        let mut button_collision = CollisionShape3D::new_alloc();

        let mut button_mesh_inner = BoxMesh::new_gd();
        button_mesh_inner.set_size(size);
        button_mesh_inner.set_material(&material);
        button_mesh.set_mesh(&button_mesh_inner);

        let mut button_collision_inner = BoxShape3D::new_gd();
        button_collision_inner.set_size(size);
        button_collision.set_shape(&button_collision_inner);

        body.add_child(&button_mesh);
        body.add_child(&button_collision);

        // panel button collision layer (layer 8)
        body.set_collision_layer(1 << 7);

        for i in [(-size.z / 2.) - 0.001, (size.z / 2.) + 0.001].iter() {
            let mut label = Self::new_label();
            label.set_text(&text.clone());
            label.set_position(Vector3::new(0., 0., *i));
            if i.is_sign_negative() {
                label.set_rotation(Vector3::new(0., PI, 0.));
            };
            body.add_child(&label);
        }

        body
    }

    #[func]
    pub fn clear_looking_at(&self) {
        let looking_at = self
            .looking_at
            .lock()
            .expect("failed to get looking_at lock");
        if let Some(looking_at) = *looking_at {
            let currently_selected = self
                .currently_selected
                .read()
                .expect("failed to read QuestionPanel::currently_selected");
            let material_prev =
                load::<StandardMaterial3D>(if looking_at == self.question.choices.len() as i64 {
                    materials::QP_SUBMIT
                } else if currently_selected.contains(&looking_at) {
                    materials::QP_SELECTED
                } else {
                    materials::QP_NORMAL
                });
            self.get_button_mesh(self.get_button_from_idx(looking_at).expect("button exists"))
                .set_material_override(&material_prev);
        }
    }
    #[func]
    pub fn set_looking_at(&self, obj: Gd<Object>) {
        if *self.submitted.lock().expect("failed to get submitted lock") {
            return;
        }
        let button_index = self.button_objects.iter().position(|x| obj.eq(&x));
        if let Some(idx) = button_index.map(|x| x as i64) {
            self.clear_looking_at();
            let currently_selected = self
                .currently_selected
                .read()
                .expect("failed to read QuestionPanel::currently_selected");
            let material_new =
                load::<StandardMaterial3D>(if idx == self.question.choices.len() as i64 {
                    materials::QP_SUBMIT
                } else if currently_selected.contains(&idx) {
                    materials::QP_SELECTED
                } else {
                    materials::QP_HOVERED
                });

            let button = self.get_button_from_idx(idx).expect("button exists");
            self.get_button_mesh(button.clone())
                .set_material_override(&material_new);

            let mut looking_at = self
                .looking_at
                .lock()
                .expect("failed to get looking_at lock");
            // play hover sound if they just started looking at the button
            if *looking_at != Some(idx) {
                let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
                let scene_manager = scene_manager.bind();
                let sfx = scene_manager.get_sfx_controller();
                let sfx = sfx.bind();
                sfx.play_qp_hover(button.get_global_position());

                *looking_at = Some(idx);
            }
        }
    }

    pub fn submit(&self) {
        let currently_selected = self
            .currently_selected
            .read()
            .expect("failed to read QuestionPanel::currently_selected");

        if currently_selected.len() != self.question.answers.len() {
            return;
        }

        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();
        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();

        let mut world = scene_manager
            .get_world_scene()
            .expect("question panel is in world");

        {
            let mut submitted = self.submitted.lock().expect("failed to get submitted lock");
            *submitted = true
        }

        let time_taken = world.bind().time_elapsed - self.start_time_ms;
        let answered_question = self
            .question
            .to_answered(currently_selected.clone(), time_taken);

        for answer in answered_question.given_answers.iter_shared() {
            let correct = answered_question.correct_answers.contains(answer);
            let button = self.get_button_from_idx(answer).expect("button exists");

            if correct {
                sfx.play_qp_correct(button.get_global_position());
            } else {
                sfx.play_qp_incorrect(button.get_global_position());
            }

            let material = load::<StandardMaterial3D>(if correct {
                materials::QP_CHOICE_CORRECT
            } else {
                materials::QP_CHOICE_INCORRECT
            });

            sfx.play_qp_select(button.get_global_position());

            self.get_button_mesh(button)
                .set_material_override(&material);
        }

        world.bind_mut().question_answered(answered_question);
    }

    pub fn handle_click(&self) {
        let looking_at = self
            .looking_at
            .lock()
            .expect("failed to get looking_at lock");
        let Some(looking_at) = *looking_at else {
            return;
        };
        if *self.submitted.lock().expect("failed to get submitted lock") {
            return;
        }

        let button = self.get_button_from_idx(looking_at).expect("button exists");

        let scene_manager = SceneManager::get_manager(self.base().clone().upcast());
        let scene_manager = scene_manager.bind();
        let sfx = scene_manager.get_sfx_controller();
        let sfx = sfx.bind();
        sfx.play_qp_select(button.get_global_position());

        if looking_at == (self.button_objects.len() as i64 - 1) {
            godot_print!("attempting to submit");
            self.submit();
        } else {
            let mut currently_selected = self
                .currently_selected
                .write()
                .expect("failed to get writer for QuestionPanel::currently_selected");

            // dont allow selecting the same thing twice.
            if currently_selected.contains(&looking_at) {
                return;
            }

            if currently_selected.len() >= self.question.answers.len() {
                let removed = currently_selected.pop_front();
                if let Some(idx) = removed {
                    let material = load::<StandardMaterial3D>(if removed == Some(looking_at) {
                        materials::QP_HOVERED
                    } else {
                        materials::QP_NORMAL
                    });
                    self.get_button_mesh(self.get_button_from_idx(idx).expect("button exists"))
                        .set_material_override(&material);
                }
            }
            currently_selected.push_back(looking_at);
            let selected = load::<StandardMaterial3D>(materials::QP_SELECTED);
            self.get_button_mesh(button)
                .set_material_override(&selected);
        }
    }

    #[func]
    /// The returned label must be added to the scene otherwise it will be leaked memory.
    fn new_label() -> Gd<Label3D> {
        let mut label = Label3D::new_alloc();
        label.set_render_priority(3);
        label.set_font_size(48);
        label.set_width(TEXT_WIDTH_PX);
        label.set_autowrap_mode(AutowrapMode::WORD);
        label.set_draw_flag(DrawFlags::DOUBLE_SIDED, false);
        label
    }

    #[func]
    pub fn get_button_from_idx(&self, button: i64) -> Option<Gd<AnimatableBody3D>> {
        self.button_objects
            .get(button as usize)?
            .clone()
            .try_cast::<AnimatableBody3D>()
            .ok()
    }

    #[func]
    pub fn get_button_mesh(&self, button: Gd<AnimatableBody3D>) -> Gd<MeshInstance3D> {
        button
            .get_child(0)
            .unwrap()
            .try_cast::<MeshInstance3D>()
            .unwrap()
    }

    #[func]
    pub fn get_button_shape(&self, button: Gd<AnimatableBody3D>) -> Gd<MeshInstance3D> {
        button
            .get_child(0)
            .unwrap()
            .try_cast::<MeshInstance3D>()
            .unwrap()
    }
}
