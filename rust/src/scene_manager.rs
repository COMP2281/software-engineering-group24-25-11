use cfg_if::cfg_if;
#[cfg(target_arch = "wasm32")]
use godot::classes::WebXrInterface;
#[cfg(not(target_arch = "wasm32"))]
use godot::classes::XrInterface;
use godot::{
    classes::{input::MouseMode, Control, DisplayServer, Time, XrServer},
    prelude::*,
};

use crate::{
    character::{normal::Player3D, vr::PlayerVR},
    menu::{completion_screen::CompletionScreen, title::TitleScreen},
    question_bank::{Course, Mode, QuestionBank},
    scenes::world::WorldScene,
    sfx::SFXController,
};

#[derive(Debug, Default, Clone, Copy, GodotConvert, Var, Export)]
#[godot(via = u8)]
pub enum InputMode {
    VR,
    #[default]
    Normal,
}

#[allow(unused)]
#[derive(GodotClass)]
#[class( base=Node)]
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
    fn init(base: Base<Self::Base>) -> Self {
        #[allow(unused_mut)]
        let mut manager = Self {
            #[cfg(not(target_arch = "wasm32"))]
            xr_interface: XrServer::singleton().find_interface("OpenXR"),
            #[cfg(target_arch = "wasm32")]
            webxr_interface: XrServer::singleton()
                .find_interface("WebXR")
                .map(|interface| {
                    interface
                        .try_cast::<WebXrInterface>()
                        .expect("should be a webxr interface")
                }),

            vr_supported: false,
            input_mode: None,
            course: None,
            difficulty: None,
            base,
        };

        DisplayServer::singleton().window_set_title("Spacedroid");

        cfg_if!(
            if #[cfg(target_arch = "wasm32")] {
                let session_supported = manager.base().callable("webxr_session_supported");
                let session_started = manager.base().callable("webxr_session_started");
                let session_ended = manager.base().callable("webxr_session_ended");
                let session_failed = manager.base().callable("webxr_session_failed");
                if let Some(ref mut webxr_interface) = manager.webxr_interface {

                    // WebXR works with signals, so we need to deal with them as they are triggered.
                    webxr_interface.connect("session_supported", &session_supported);
                    webxr_interface.connect("session_started", &session_started);
                    webxr_interface.connect("session_ended", &session_ended);
                    webxr_interface.connect("session_failed", &session_failed);

                    webxr_interface.is_session_supported("immersive-vr");
                }
            }
        );

        manager
    }
    fn ready(&mut self) {
        self.get_title_screen()
            .expect("initial game should have a title screen")
            .grab_focus();
    }
}
#[godot_api]
impl SceneManager {
    #[func]
    pub fn get_root_base(base: Gd<Node>) -> Gd<Node> {
        base.get_node_as::<Node>("/root/Root")
    }

    #[func]
    pub fn get_manager(base: Gd<Node>) -> Gd<SceneManager> {
        Self::get_root_base(base).get_node_as::<SceneManager>("SceneManager")
    }

    #[func]
    pub fn get_root(&self) -> Gd<Node> {
        Self::get_root_base(self.base().clone().upcast())
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
    pub fn exit_world(&mut self) {
        godot_print!("exiting the world");
        let mut title_scene = self.create_title_screen();
        self.swap_scene(title_scene.clone().upcast());
        title_scene.grab_focus();

        match self.input_mode {
            Some(InputMode::VR) => {
                cfg_if!(
                    if #[cfg(target_arch = "wasm32")] {
                        if let Some(ref mut webxr_interface) = self.webxr_interface {
                            if webxr_interface.is_initialized() {
                                webxr_interface.uninitialize()
                            }
                        }
                    }
                );
                godot_print!("Viewport no longer using XR.");
            }
            None => godot_warn!("called exit world without setting a target mode"),
            _ => {}
        }
    }

    #[func]
    pub fn resume_game(&self) {
        let mut world = self
            .get_world_scene()
            .expect("resume called with a valid world");
        world.bind_mut().previous_time = Time::singleton().get_ticks_msec();
        world.get_tree().expect("is in scene tree").set_pause(false);

        self.hide_pause_menu();
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }
    #[func]
    pub fn pause_game(&self) {
        self.get_world_scene()
            .expect("resume called with a valid world")
            .get_tree()
            .expect("is in scene tree")
            .set_pause(true);
        self.show_pause_menu();
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::VISIBLE);
    }

    // FIXME: make these load the scene maybe
    #[func]
    pub fn show_pause_menu(&self) {
        let mut pause_screen = self.create_pause_screen();
        self.get_root().add_child(&pause_screen);
        pause_screen.grab_focus();
    }
    #[func]
    pub fn hide_pause_menu(&self) {
        if let Some(pause_screen) = self.get_pause_screen() {
            self.get_root().remove_child(&pause_screen);
        } else {
            godot_warn!(
                "scene manager: called hide_pause_menu without a pause menu being available"
            );
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
    pub fn enter_vr_world(&self) {
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
        }
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
            if let Some(title_screen) = self.get_title_screen() {
                title_screen
                    .bind()
                    .show_message("VR is not supported on this device.".into())
            } else {
                os.alert("vr is not supported");
            }
            return;
        }

        let Some(ref mut webxr_interface) = &mut self.webxr_interface else {
            return;
        };

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
            godot_print!(
                "immersive-vr is not supported for this session, disabling vr functionality."
            );
            return;
        }
        self.vr_supported = true;
    }
    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_started(&self) {
        godot_print!("immersive-vr session started, attempting to enter world...");

        self.get_root()
            .get_viewport()
            .expect("game scene has a viewport")
            .set_use_xr(true);

        if let Some(world) = self.get_world_scene() {
            self.resume_game();
        } else {
            self.enter_vr_world();
        }
    }

    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_ended(&mut self) {
        if let Some(ref mut webxr_interface) = self.webxr_interface {
            if webxr_interface.is_initialized() {
                webxr_interface.uninitialize()
            }
        };
        self.get_root()
            .get_viewport()
            .expect("game scene has a viewport")
            .set_use_xr(false);
        godot_print!("Viewport stopped using XR.");
        if let Some(world) = self.get_world_scene() {
            self.pause_game();
        } else if let Some(completion_screen) = self.get_completion_screen() {
            godot_print!("completed the game");
        } else {
            self.exit_world();
            self.get_title_screen()
                .expect("should be in title screen")
                .bind()
                .show_message("WebXR session has ended.".into())
        }
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
            if webxr_interface.is_initialized() {
                webxr_interface.uninitialize()
            }
        }
    }

    #[func]
    fn create_title_screen(&self) -> Gd<Control> {
        let mut title_scene =
            load::<PackedScene>(crate::resources::TITLE_SCENE).instantiate_as::<Control>();
        title_scene.set_name("TitleScreen");
        title_scene
    }

    #[func]
    fn create_pause_screen(&self) -> Gd<Control> {
        let mut pause_scene =
            load::<PackedScene>(crate::resources::PAUSE_SCENE).instantiate_as::<Control>();
        pause_scene.set_name("PauseMenu");
        pause_scene
    }

    pub fn show_completion_screen(&mut self, question_bank: QuestionBank, time_elapsed: u64) {
        let mut completion_scene = CompletionScreen::new_screen(question_bank, time_elapsed);
        self.swap_scene(completion_scene.clone().upcast());
        completion_scene.grab_focus();

        match self.input_mode {
            Some(InputMode::VR) => {
                cfg_if!(
                    if #[cfg(target_arch = "wasm32")] {
                        if let Some(ref mut webxr_interface) = self.webxr_interface {
                            if webxr_interface.is_initialized() {
                                webxr_interface.uninitialize()
                            }
                        }
                    }
                );
                self.get_root()
                    .get_viewport()
                    .expect("game scene has a viewport")
                    .set_use_xr(false);
                godot_print!("Viewport no longer using XR.");
            }
            None => godot_warn!("called exit world without setting a target mode"),
            _ => {}
        }
    }

    #[func]
    pub fn get_title_screen(&self) -> Option<Gd<TitleScreen>> {
        self.get_root()
            .get_node_or_null("TitleScreen")?
            .try_cast::<TitleScreen>()
            .ok()
    }

    #[func]
    pub fn get_pause_screen(&self) -> Option<Gd<Control>> {
        self.get_root()
            .get_node_or_null("PauseMenu")?
            .try_cast::<Control>()
            .ok()
    }
    #[func]
    pub fn get_completion_screen(&self) -> Option<Gd<CompletionScreen>> {
        self.get_root()
            .get_node_or_null("CompletionScreen")?
            .try_cast::<CompletionScreen>()
            .ok()
    }
    #[func]
    pub fn close_completion_screen(&self) {
        let mut title_scene = self.create_title_screen();
        self.swap_scene(title_scene.clone().upcast());
        title_scene.grab_focus();
    }

    #[func]
    pub fn get_world_scene(&self) -> Option<Gd<WorldScene>> {
        self.get_root()
            .get_node_or_null("World")?
            .try_cast::<WorldScene>()
            .ok()
    }

    #[func]
    pub fn get_sfx_controller(&self) -> Gd<SFXController> {
        self.get_root()
            .get_node_as::<SFXController>("SFXController")
    }

    #[func]
    fn swap_scene(&self, scene: Gd<Node>) {
        godot_print!("swapping scenes to: {}", scene.get_name());
        let mut root_node = self.get_root();
        root_node
            .get_tree()
            .expect("root node is in scene tree")
            .set_pause(false);

        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::VISIBLE);

        if let Some(world_scene) = self.get_world_scene() {
            godot_print!("removing world scene");
            root_node.remove_child(&world_scene);
            // stop walking sfx if it is still playing.
            self.get_sfx_controller().bind().stop_walking();
        }
        if let Some(title_screen) = self.get_title_screen() {
            godot_print!("removing title scene");
            root_node.remove_child(&title_screen);
        }
        if let Some(completion_screen) = self.get_completion_screen() {
            godot_print!("removing title scene");
            root_node.remove_child(&completion_screen);
        }
        root_node.add_child(&scene);
    }
}
