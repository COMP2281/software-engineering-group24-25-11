use std::str::FromStr;

use crate::{
    character::{normal::Player3D, vr::PlayerVR},
    scene_manager::SceneManager,
    scenes::world::WorldScene,
};
use cfg_if::cfg_if;
use godot::{
    classes::{
        display_server::VSyncMode, xr_interface, Button, DisplayServer, IButton, InputEvent, Os,
        Tween, WebXrInterface, XrCamera3D, XrServer,
    },
    obj::NewGd,
    prelude::*,
};

#[derive(Default)]
enum Actions {
    #[default]
    EnterVR,
    Enter3D,
    Tutorial,
    Options,
    Credits,
    Quit,
    Unknown,
}
impl From<i64> for Actions {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::EnterVR,
            1 => Self::Enter3D,
            2 => Self::Tutorial,
            3 => Self::Options,
            4 => Self::Credits,
            5 => Self::Quit,
            _ => Self::Unknown,
        }
    }
}

const TRANSITION_DURATION: f64 = 0.2;
const NORMAL_COLOR: Color = Color {
    r: 1.,
    g: 1.,
    b: 1.,
    a: 1.,
};
const HOVER_COLOR: Color = Color {
    r: 2.,
    g: 2.,
    b: 2.,
    a: 1.,
};

#[derive(GodotClass)]
#[class(base=Button)]
struct CustomMenuButton {
    #[export(enum = (EnterVR, Enter3D, Tutorial, Options, Credits, Quit))]
    action: i64,
    base: Base<Button>,
}

#[godot_api]
impl IButton for CustomMenuButton {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            // tween: None,
            action: Default::default(),
            base,
        }
    }

    fn pressed(&mut self) {
        let mut scene_tree = self.base().get_tree().expect("able to get the scene tree");
        let mut os = Os::singleton();
        let mut root_node = scene_tree
            .get_root()
            .unwrap()
            .get_node_as::<Node>("/root/Root");

        let mut title_screen = root_node
            .find_child("TitleScreen".into())
            .expect("custom menu button used outside of title screen");
        let mut scene_manager = root_node
            .find_child("SceneManager".into())
            .expect("SceneManager not present at the root of the scene")
            .try_cast::<SceneManager>()
            .expect("path /SceneManager is not a SceneManager!");
        let mut scene_manager = scene_manager.bind_mut();

        match self.get_action().into() {
            Actions::EnterVR => {
                godot_print!("titlescreen: Attempting to enter world using immersive VR.");
                cfg_if! {
                    if #[cfg(target_arch = "wasm32")] {
                        godot_print!("initialising webxr");
                        scene_manager.init_web_xr();
                    } else {
                        godot_print!("initialising openxr");
                        scene_manager.init_open_xr();
                    }
                };

                // let xr_server = XrServer::singleton();
                // if os.has_feature("web".into()) {
                //     godot_print!("titlescreen: detected web, trying to find WebXR interface.");
                //     let xr_interace = xr_server.find_interface("WebXR".into());
                //     let Some(xr_interface) = xr_interace else {
                //         os.alert("failed to find web xr interface!".into());
                //         return;
                //     };
                //     let mut xr_interface = xr_interface
                //         .try_cast::<WebXrInterface>()
                //         .expect("should be a webxr interface");
                //     godot_print!("WebXR: Configuring interface");
                //     if !xr_interface.is_initialized() && !xr_interface.initialize() {
                //         os.alert("Could not initialise VR interface!".into());
                //         return;
                //     }
                //
                //     let base = self.base().to_godot();
                //     self.base_mut().connect(
                //         "is_session_supported".into(),
                //         Callable::from_object_method(&base, "immersive_xr_supported"),
                //     );
                //     xr_interface.set_session_mode("immersive-vr".into());
                //     xr_interface.set_requested_reference_space_types(
                //         "bounded-floor, local-floor, local".into(),
                //     );
                //     xr_interface.set_required_features("local-floor".into());
                //     xr_interface.set_optional_features("bounded-floor, hand-tracking".into());
                //
                //     xr_interface.is_session_supported("immersive-vr".into());
                //
                //     // display_server.window_set_vsync_mode(VSyncMode::DISABLED);
                // } else {
                //     godot_print!("titlescreen: detected desktop, trying to find OpenXR interface.");
                //     let xr_interace = xr_server.find_interface("OpenXR".into());
                //     let Some(xr_interface) = xr_interace else {
                //         os.alert("failed to find open xr interface!".into());
                //         return;
                //     };
                // }
            }
            Actions::Enter3D => {
                godot_print!("entering as a normal 3d game");
                let world_scene = load::<PackedScene>("res://scenes/game/world.tscn")
                    .instantiate_as::<WorldScene>();
                let character_3d = load::<PackedScene>("res://scenes/character/character_3d.tscn")
                    .instantiate_as::<Player3D>();

                let mut player = world_scene.get_node_as::<Node>("Player");
                player.add_child(&character_3d);

                root_node.remove_child(title_screen);
                root_node.add_child(world_scene);

                godot_print!("Done entering");
            }
            _ => {}
        }
    }
}

#[godot_api]
impl CustomMenuButton {
    #[func]
    fn immersive_xr_supported(&mut self, session_mode: GString, supported: bool) {
        if session_mode != "immersive-vr".into() || !supported {
            return;
        }
        let mut scene_tree = self.base().get_tree().expect("able to get the scene tree");
        let mut os = Os::singleton();

        let mut root_node = scene_tree
            .get_root()
            .unwrap()
            .get_node_as::<Node>("/root/Root");

        let mut title_screen = root_node
            .find_child("TitleScreen".into())
            .expect("custom menu button used outside of title screen");

        let xr_server = XrServer::singleton();
        let xr_interace = xr_server.find_interface("WebXR".into());
        let Some(xr_interface) = xr_interace else {
            os.alert("failed to find web xr interface!".into());
            return;
        };
        let mut xr_interface = xr_interface
            .try_cast::<WebXrInterface>()
            .expect("should be a webxr interface");

        if !xr_interface.is_passthrough_supported() && !xr_interface.start_passthrough() {
            os.alert("Could not start VR passthrough!".into());
            return;
        }
        root_node
            .get_viewport()
            .expect("game scene has a viewport")
            .set_use_xr(true);

        let world_scene =
            load::<PackedScene>("res://scenes/game/world.tscn").instantiate_as::<WorldScene>();
        let character_vr = load::<PackedScene>("res://scenes/character/character_vr.tscn")
            .instantiate_as::<PlayerVR>();

        let mut player = world_scene.get_node_as::<Node>("Player");
        player.add_child(&character_vr);

        root_node.remove_child(title_screen);
        root_node.add_child(world_scene);
    }
}
