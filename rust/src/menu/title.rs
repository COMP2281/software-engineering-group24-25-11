use crate::{character::normal::Player3D, scene_manager::SceneManager, scenes::world::WorldScene};
use cfg_if::cfg_if;
use godot::{
    classes::{Button, IButton},
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
        let scene_tree = self.base().get_tree().expect("able to get the scene tree");
        let mut root_node = scene_tree
            .get_root()
            .unwrap()
            .get_node_as::<Node>("/root/Root");

        let title_screen = root_node
            .find_child(&GString::from("TitleScreen"))
            .expect("custom menu button used outside of title screen");
        let mut scene_manager = root_node
            .find_child(&GString::from("SceneManager"))
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
            }
            Actions::Enter3D => {
                // TODO: move to scene manager
                godot_print!("entering as a normal 3d game");
                let world_scene = load::<PackedScene>("res://scenes/game/world.tscn")
                    .instantiate_as::<WorldScene>();
                let character_3d = load::<PackedScene>("res://scenes/character/character_3d.tscn")
                    .instantiate_as::<Player3D>();

                let mut player = world_scene.get_node_as::<Node>("Player");
                player.add_child(&character_3d);

                root_node.remove_child(&title_screen);
                root_node.add_child(&world_scene);

                godot_print!("Done entering");
            }
            Actions::Quit => {
                let mut scene_tree = self
                    .base()
                    .get_tree()
                    .expect("main menu is in the scene tree");
                scene_tree.quit();
            }
            _ => {}
        }
    }
}
