use std::{f32::consts::PI, str::FromStr};

use godot::{
    classes::{
        label_3d::DrawFlags, text_server::AutowrapMode, AnimatableBody3D, BoxMesh, BoxShape3D,
        CollisionShape3D, Label3D, MeshInstance3D, StandardMaterial3D, StaticBody3D,
    },
    obj::WithBaseField,
    prelude::*,
};

use crate::{question_bank::Question, scenes::world::WorldScene};

#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
pub struct QuestionPanel {
    question: Question,

    currently_selected: Array<i64>,
    /// 0..n-1 are the choices buttons, whereas the last object is the submit button.
    button_objects: Array<Gd<Object>>,
    // index of the button currently being looked at
    looking_at: Option<i64>,
    // whether the panel has already been submitted, if it has no actions are allowed on it.
    submitted: bool,
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
    pub fn create_panel(question: Question) -> Gd<Self> {
        let mut panel = Gd::from_init_fn(|base| Self {
            question,

            currently_selected: Array::default(),
            button_objects: Array::default(),
            looking_at: None,
            submitted: false,

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

        let hot_blued_steel = load::<StandardMaterial3D>(crate::resources::MAT_HOT_BLUED_STEEL);

        let mut panel_mesh_inner = BoxMesh::new_gd();
        panel_mesh_inner.set_size(PANEL_SIZE);
        panel_mesh_inner.set_material(&hot_blued_steel);
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
        let gold = load::<StandardMaterial3D>(crate::resources::MAT_COPPER);
        for (idx, choice) in self.question.choices.clone().iter().enumerate() {
            let mut btn = self.create_button(gold.clone(), GString::from(choice), BUTTON_SIZE);
            btn.set_position(Vector3::new(
                0.,
                (PANEL_SIZE.y / 2.) - 2.5 - ((BUTTON_SIZE.y + BUTTON_SPACER) * idx as f32),
                0.,
            ));

            self.button_objects.push(&btn.clone().upcast());
            self.base_mut().add_child(&btn);
        }
    }
    #[func]
    fn add_submit_button(&mut self) {
        let gold = load::<StandardMaterial3D>(crate::resources::MAT_GOLD);
        let mut btn = self.create_button(gold, SUBMIT_TEXT.into(), BUTTON_SIZE);

        btn.set_position(Vector3::new(0., -(PANEL_SIZE.y / 2.) + 1., 0.));

        self.button_objects.push(&btn.clone().upcast());
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
    /// Returns None if an answer has not been selected yet, otherwise return whether
    /// the answer was correct.
    #[func]
    pub fn submit(&mut self) {
        if self.currently_selected.len() != self.question.answers.len() {
            return;
        }
        let scene_tree = self
            .base()
            .get_tree()
            .expect("question panel is in scene tree");
        let mut world = WorldScene::get_world(scene_tree).expect("question panel is in world");

        self.submitted = true;

        let answered_question = self
            .question
            .to_answered(self.currently_selected.clone(), 0);

        world.bind_mut().question_answered(answered_question);
    }

    #[func]
    pub fn set_looking_at(&mut self, obj: Gd<Object>) {
        let button_index = self.button_objects.iter_shared().position(|x| obj.eq(&x));
        self.looking_at = button_index.map(|x| x as i64);
    }

    #[func]
    pub fn handle_click(&mut self) {
        let Some(looking_at) = self.looking_at else {
            return;
        };
        if self.submitted {
            return;
        };

        if looking_at == (self.button_objects.len() as i64 - 1) {
            godot_print!("attempting to submit");
            self.submit();
        } else {
            if self.currently_selected.len() >= self.question.answers.len() {
                let removed = self.currently_selected.pop_front();
                if let Some(removed) = removed {
                    let empty = load::<StandardMaterial3D>(crate::resources::MAT_EMPTY);
                    self.get_button_mesh(self.get_button_from_idx(removed).expect("button exists"))
                        .set_material_override(&empty);
                }
            }
            self.currently_selected.push(looking_at);

            godot_print!("currently selected {:?}", self.currently_selected);

            let selected = load::<StandardMaterial3D>(crate::resources::MAT_SELECTED);
            self.get_button_mesh(self.get_button_from_idx(looking_at).expect("button exists"))
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
