use cfg_if::cfg_if;
#[cfg(target_arch = "wasm32")]
use godot::classes::WebXrInterface;
#[cfg(not(target_arch = "wasm32"))]
use godot::classes::XrInterface;
use godot::{
    classes::{Control, XrServer},
    prelude::*,
};

use crate::{
    character::{normal::Player3D, vr::PlayerVR},
    question_bank::{Course, Mode},
    scenes::world::WorldScene,
};

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export)]
#[godot(via = u8)]
pub enum InputMode {
    VR,
    #[default]
    Normal,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SceneManager {
    #[cfg(not(target_arch = "wasm32"))]
    xr_interface: Option<Gd<XrInterface>>,
    #[cfg(target_arch = "wasm32")]
    webxr_interface: Option<Gd<WebXrInterface>>,
    vr_supported: bool,

    input_mode: Option<InputMode>,
    course: Option<Course>,
    difficulty: Option<Mode>,

    base: Base<Node>,
}

#[godot_api]
impl INode for SceneManager {
    fn ready(&mut self) {
        self.vr_supported = false;
        cfg_if!(
            if #[cfg(not(target_arch = "wasm32"))] {
                self.xr_interface = XrServer::singleton().find_interface(&GString::from("OpenXR"));
            } else {
                let mut webxr_interface = XrServer::singleton().find_interface(&GString::from("WebXR"));
                if let Some(mut webxr_interface) = webxr_interface {
                    let mut webxr_interface = webxr_interface
                        .try_cast::<WebXrInterface>()
                        .expect("should be a webxr interface");

                    let session_supported = self.base().callable("webxr_session_supported");
                    let session_started = self.base().callable("webxr_session_started");
                    let session_ended = self.base().callable("webxr_session_ended");
                    let session_failed = self.base().callable("webxr_session_failed");
                    webxr_interface.connect(&StringName::from("session_supported"), &session_supported);
                    webxr_interface.connect(&StringName::from("session_started"), &session_started);
                    webxr_interface.connect(&StringName::from("session_ended"), &session_ended);
                    webxr_interface.connect(&StringName::from("session_failed"), &session_failed);

                    webxr_interface.is_session_supported(&GString::from("immersive-vr"));
                    self.webxr_interface = Some(webxr_interface);
                }
            }
        );
    }
}
#[godot_api]
impl SceneManager {
    #[func]
    fn create_alert(&mut self, message: GString) {
        godot_warn!("todo: alert! message: {}", message);
    }
    #[func]
    fn get_root(&self) -> Gd<Node> {
        let scene_tree = self
            .base()
            .get_tree()
            .expect("scene manager is in the scene tree");
        scene_tree
            .get_root()
            .unwrap()
            .get_node_as::<Node>("/root/Root")
    }
    #[func]
    pub fn set_input_mode(&mut self, target: InputMode) {
        self.input_mode = Some(target);
    }
    #[func]
    pub fn set_course(&mut self, course: Course) {
        self.course = Some(course);
    }
    #[func]
    pub fn set_difficulty(&mut self, mode: Mode) {
        self.difficulty = Some(mode);
    }

    #[func]
    pub fn enter_world(&mut self) {
        match self.input_mode {
            Some(InputMode::VR) => {
                cfg_if!(
                    if #[cfg(target_arch = "wasm32")] {
                        self.init_web_xr();
                    } else {
                        self.init_open_xr();
                    }
                );
            }
            Some(InputMode::Normal) => self.init_3d(),
            None => godot_warn!("called enter world without setting a target mode"),
        }
    }

    #[func]
    pub fn init_3d(&mut self) {
        if let (Some(course), Some(mode)) = (self.course, self.difficulty) {
            let world_scene = WorldScene::create_world_with_bank(course, mode);
            let character_3d = load::<PackedScene>(crate::resources::NORMAL_CHARACTER)
                .instantiate_as::<Player3D>();
            let mut player = world_scene.get_node_as::<Node>("Player");
            player.add_child(&character_3d);

            self.swap_scene(world_scene.upcast());
            godot_print!("Done entering world.");
        } else {
            godot_warn!("called init_3d without setting a course and difficulty")
        }
    }

    #[func]
    pub fn enter_vr_world(&mut self) {
        if let (Some(course), Some(mode)) = (self.course, self.difficulty) {
            let root_node = self.get_root();

            godot_print!("entering the vr world");
            let world_scene = WorldScene::create_world_with_bank(course, mode);
            let character_vr =
                load::<PackedScene>(crate::resources::VR_CHARACTER).instantiate_as::<PlayerVR>();

            let mut player = world_scene.get_node_as::<Node>("Player");
            player.add_child(&character_vr);

            self.swap_scene(world_scene.upcast());
            godot_print!("Done entering vr world.");

            root_node
                .get_viewport()
                .expect("game scene has a viewport")
                .set_use_xr(true);
            godot_print!("Viewport now using XR.");
        }
    }

    #[func]
    pub fn exit_vr_world(&mut self) {
        let root_node = self.get_root();

        godot_print!("exiting the vr world");

        let mut title_scene =
            load::<PackedScene>(crate::resources::TITLE_SCENE).instantiate_as::<Control>();
        title_scene.set_name(&GString::from("TitleScreen"));

        self.swap_scene(title_scene.upcast());

        root_node
            .get_viewport()
            .expect("game scene has a viewport")
            .set_use_xr(false);
        godot_print!("Viewport no longer using XR.");
    }
    // OpenXR
    #[func]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn init_open_xr(&mut self) {}
    // WebXR
    #[func]
    #[cfg(target_arch = "wasm32")]
    pub fn init_web_xr(&mut self) {
        let mut os = godot::classes::Os::singleton();
        if !self.vr_supported {
            os.alert(&GString::from("vr is not supported"));
            return;
        }

        let Some(ref mut webxr_interface) = &mut self.webxr_interface else {
            return;
        };

        // WebXR works with signals, so we need to deal with them as they are triggered.
        // TODO: make custom class for these

        webxr_interface.set_requested_reference_space_types(&GString::from("local-floor, local"));
        webxr_interface.set_required_features(&GString::from("local-floor"));
        webxr_interface.set_session_mode(&GString::from("immersive-vr"));

        if !webxr_interface.initialize() {
            godot_print!("failed to initialise webxr interface, exiting...");
            os.alert(&GString::from("Could not initialise VR interface!"));
        }
    }

    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_supported(&mut self, session_mode: GString, supported: bool) {
        if !supported || session_mode != "immersive-vr".into() {
            godot_warn!(
                "immersive-vr is not supported for this session, disabling vr functionality."
            );
            return;
        }
        self.vr_supported = true;
    }
    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_started(&mut self) {
        let mut os = godot::classes::Os::singleton();
        godot_print!("immersive-vr session started, attempting to enter world...");

        self.enter_vr_world();
    }
    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_ended(&mut self) {
        let mut os = godot::classes::Os::singleton();
        os.alert(&GString::from("session ended"));
        godot_print!("immersive-vr session ended, attempting to exit world...");
        self.exit_vr_world();

        // if let Some(ref mut webxr_interface) = self.webxr_interface {
        //     webxr_interface.uninitialize();
        //     self.webxr_interface = None
        // };
    }

    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_failed(&mut self, message: GString) {
        let mut os = godot::classes::Os::singleton();
        godot_warn!("failed to start webxr session: {}", message);
        os.alert(&GString::from(format!(
            "failed to start webxr session: {}",
            message
        )));

        if let Some(ref mut webxr_interface) = self.webxr_interface {
            webxr_interface.uninitialize();
            self.webxr_interface = None
        }
    }

    #[func]
    fn swap_scene(&mut self, scene: Gd<Node>) {
        godot_print!("swapping scenes");
        let mut root_node = self.get_root();
        if let Some(world_scene) =
            root_node.get_node_or_null(&NodePath::from("/root/Root/WorldScene"))
        {
            godot_print!("removing world scene");
            root_node.remove_child(&world_scene);
        }
        if let Some(title_screen) =
            root_node.get_node_or_null(&NodePath::from("/root/Root/TitleScreen"))
        {
            godot_print!("removing title scene");
            root_node.remove_child(&title_screen);
        }
        root_node.add_child(&scene);
    }
}
