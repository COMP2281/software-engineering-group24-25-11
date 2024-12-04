use crate::init::RootScene;
use godot::{
    classes::{
        display_server::VSyncMode, Button, DisplayServer, IButton, Os, XrCamera3D, XrServer,
    },
    prelude::*,
};

enum Actions {
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

#[derive(GodotClass)]
#[class(init, base=Button)]
struct CustomMenuButton {
    #[export(enum = (EnterVR, Enter3D, Tutorial, Options, Credits, Quit))]
    action: i64,
    base: Base<Button>,
}

#[godot_api]
impl IButton for CustomMenuButton {
    fn pressed(&mut self) {
        let mut scene_tree = self.base().get_tree().expect("able to get the scene tree");
        let game_scene = load::<PackedScene>("res://game.tscn");
        let game_root = game_scene.instantiate_as::<RootScene>();
        let mut os = Os::singleton();
        //see if web: os.has_feature("web".into());
        match self.get_action().into() {
            Actions::EnterVR => {
                godot_print!("entering as a vr game");

                let xr_server = XrServer::singleton();
                let mut display_server = DisplayServer::singleton();
                let xr_interface = if os.has_feature("web".into()) {
                    xr_server.find_interface("WebXR".into())
                } else {
                    xr_server.find_interface("OpenXR".into())
                };
                if xr_interface.is_none() {
                    os.alert("Could not find any VR interfaces!".into());
                    return;
                }
                let mut interface = xr_interface.unwrap();
                if !os.has_feature("web".into()) {
                    godot_print!("OpenXR: Configuring interface");
                    if !interface.is_initialized() && !interface.initialize() {
                        os.alert("Could not initialise VR interface!".into());
                        return;
                    }
                    //interface.connect("session_begun".into(), || _);
                    //interface.connect("session_visible".into(), || _);
                    //interface.connect("session_focused".into(), || _);
                    if interface.is_passthrough_supported() {
                        interface.start_passthrough();
                    }
                    display_server.window_set_vsync_mode(VSyncMode::DISABLED);
                    game_root
                        .get_viewport()
                        .expect("game scene has a viewport")
                        .set_use_xr(true);
                }

                let mut camera =
                    game_root.get_node_as::<XrCamera3D>("Player/XROrigin3D/XRCamera3D");
                camera.make_current();
                godot_print!("camera is current {:?}", camera.is_current());

                scene_tree.change_scene_to_packed(game_scene);
            }
            Actions::Enter3D => {
                godot_print!("entering as a normal 3d game");
                let mut camera = game_root.get_node_as::<Camera3D>("Player/Pivot/Camera3D");
                camera.make_current();
                godot_print!("camera is current {:?}", camera.is_current());
                scene_tree.change_scene_to_packed(game_scene);
            }
            _ => {}
        }
    }
}
