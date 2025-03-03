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
    scenes::world::WorldScene,
};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct SceneManager {
    #[cfg(not(target_arch = "wasm32"))]
    xr_interface: Option<Gd<XrInterface>>,
    #[cfg(target_arch = "wasm32")]
    webxr_interface: Option<Gd<WebXrInterface>>,

    base: Base<Node>,
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
    pub fn enter_vr_world(&mut self) {
        let mut root_node = self.get_root();

        godot_print!("entering the vr world");
        let world_scene =
            load::<PackedScene>("res://scenes/game/world.tscn").instantiate_as::<WorldScene>();
        let character_vr = load::<PackedScene>("res://scenes/character/character_vr.tscn")
            .instantiate_as::<PlayerVR>();

        if let Some(title_screen) = root_node.find_child(&GString::from("TitleScreen")) {
            root_node.remove_child(&title_screen);
        }

        if root_node.find_child(&GString::from("WorldScene")).is_none() {
            let mut player = world_scene.get_node_as::<Node>("Player");
            player.add_child(&character_vr);
            root_node.add_child(&world_scene);
        }

        root_node
            .get_viewport()
            .expect("game scene has a viewport")
            .set_use_xr(true);
    }

    #[func]
    pub fn exit_vr_world(&mut self) {
        let mut root_node = self.get_root();

        godot_print!("exiting the vr world");

        let title_scene =
            load::<PackedScene>("res://scenes/menu/title.tscn").instantiate_as::<Control>();

        if let Some(world_scene) = root_node.find_child(&GString::from("WorldScene")) {
            root_node.remove_child(&world_scene);
        }
        if root_node
            .find_child(&GString::from("TitleScreen"))
            .is_none()
        {
            root_node.add_child(&title_scene);
        }
        root_node
            .get_viewport()
            .expect("game scene has a viewport")
            .set_use_xr(false);
    }
    // OpenXR
    #[func]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn init_open_xr(&mut self) {
        let xr_interface = XrServer::singleton().find_interface(&GString::from("OpenXR"));
        let Some(xr_interface) = xr_interface else {
            return;
        };
        self.xr_interface = Some(xr_interface);
        godot_warn!("TODO: needs implementing");
    }
    // WebXR
    #[func]
    #[cfg(target_arch = "wasm32")]
    pub fn init_web_xr(&mut self) {
        let mut os = godot::classes::Os::singleton();

        os.alert(&GString::from("initialising webxr 2"));
        let xr_interface = XrServer::singleton().find_interface(&GString::from("WebXR"));
        let Some(xr_interface) = xr_interface else {
            return;
        };
        let mut webxr_interface = xr_interface
            .try_cast::<WebXrInterface>()
            .expect("should be a webxr interface");
        self.webxr_interface = Some(webxr_interface.clone());

        // WebXR works with signals, so we need to deal with them as they are triggered.
        // TODO: move these to ready()
        let session_supported = self.base().callable("webxr_session_supported");
        let session_started = self.base().callable("webxr_session_started");
        let session_ended = self.base().callable("webxr_session_ended");
        let session_failed = self.base().callable("webxr_session_failed");
        webxr_interface.connect(&StringName::from("session_supported"), &session_supported);
        webxr_interface.connect(&StringName::from("session_started"), &session_started);
        webxr_interface.connect(&StringName::from("session_ended"), &session_ended);
        webxr_interface.connect(&StringName::from("session_failed"), &session_failed);

        os.alert(&GString::from("Configuring webxr interface"));

        webxr_interface.is_session_supported(&GString::from("immersive-vr"));
        webxr_interface.set_requested_reference_space_types(&GString::from(
            "bounded-floor, local-floor, local",
        ));
        webxr_interface.set_required_features(&GString::from("local-floor"));
        webxr_interface.set_optional_features(&GString::from("bounded-floor, hand-tracking"));

        if !webxr_interface.is_initialized() && !webxr_interface.initialize() {
            godot_print!("failed to initialise webxr interface, exiting...");
            os.alert(&GString::from("Could not initialise VR interface!"));
            return;
        }
    }

    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_supported(&mut self, session_mode: GString, supported: bool) {
        if !supported || session_mode != "immersive-vr".into() {
            godot_print!("immersive-vr is not supported in this session, exiting...");
            self.webxr_interface = None;
            return;
        }
        let Some(ref mut webxr_interface) = self.webxr_interface else {
            return;
        };
        webxr_interface.set_session_mode(&GString::from("immersive-vr"));

        godot_print!("immersive-vr is supported in this session");
        let mut os = godot::classes::Os::singleton();
        os.alert(&GString::from("session supported"));
    }
    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_started(&mut self) {
        godot_print!("immersive-vr session started, attempting to enter world...");
        self.enter_vr_world();
    }
    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_ended(&mut self) {
        godot_print!("immersive-vr session ended, attempting to exit world...");
        self.exit_vr_world();

        if let Some(ref mut webxr_interface) = self.webxr_interface {
            webxr_interface.uninitialize();
            self.webxr_interface = None
        };
    }

    #[func]
    #[cfg(target_arch = "wasm32")]
    fn webxr_session_failed(&mut self, message: GString) {
        let mut os = godot::classes::Os::singleton();
        os.alert(&GString::from(format!(
            "failed to start webxr session: {}",
            message
        )));
        godot_warn!("failed to start webxr session: {}", message);

        if let Some(ref mut webxr_interface) = self.webxr_interface {
            webxr_interface.uninitialize();
            self.webxr_interface = None
        }
    }
    #[func]
    pub fn init_3d(&mut self) {
        // FIXME: move resource paths into const file
        let scene_tree = self.base().get_tree().expect("able to get the scene tree");
        let world_scene =
            load::<PackedScene>("res://scenes/game/world.tscn").instantiate_as::<WorldScene>();
        let character_3d = load::<PackedScene>("res://scenes/character/character_3d.tscn")
            .instantiate_as::<Player3D>();

        let mut player = world_scene.get_node_as::<Node>("Player");
        player.add_child(&character_3d);

        self.swap_scene(world_scene.upcast());

        godot_print!("Done entering");
    }

    #[func]
    fn swap_scene(&mut self, scene: Gd<Node>) {
        let mut root_node = self.get_root();
        if let Some(world_scene) = root_node.find_child(&GString::from("WorldScene")) {
            root_node.remove_child(&world_scene);
        }
        if let Some(title_screen) = root_node.find_child(&GString::from("TitleScreen")) {
            root_node.remove_child(&title_screen);
        }
        root_node.add_child(&scene);
    }
    // #[func]
    // fn switch_to_world(&self, is_vr: bool) {
    //     let mut scene_tree = self.base().get_tree().expect("able to get the scene tree");
    //
    //     let mut root_node = scene_tree
    //         .get_root()
    //         .unwrap()
    //         .get_node_as::<Node>("/root/Root");
    //     let mut title_screen = root_node
    //         .find_child("TitleScreen".into())
    //         .expect("custom menu button used outside of title screen");
    //     let world_scene =
    //         load::<PackedScene>("res://scenes/game/world.tscn").instantiate_as::<WorldScene>();
    //     let character_path = if is_vr {
    //         "res://scenes/character/character_vr.tscn"
    //     } else {
    //         "res://scenes/character/character_3d.tscn"
    //     };
    //
    //     let character = load::<PackedScene>(character_path).instantiate().unwrap();
    //
    //     let mut player = world_scene.get_node_as::<Node>("Player");
    //     player.add_child(&character);
    //
    //     root_node.remove_child(title_screen);
    //     root_node.add_child(world_scene);
    //
    //     // Common setup logic...
    // }
}
