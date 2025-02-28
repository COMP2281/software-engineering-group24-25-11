// use godot::{
//     classes::{control::LayoutPreset, Control, PanelContainer, Theme, VBoxContainer},
//     prelude::*,
// };

// #[derive(GodotClass)]
// #[class(no_init, base=Control)]
// struct MessageArea {
//     title: GString,
//     message: GString,
//     base: Base<Control>,
// }

// #[godot_api]
// impl MessageArea {
//     #[func]
//     fn from_message(title: GString, message: GString) -> Gd<Self> {
//         // Function contains a single statement, the `Gd::from_init_fn()` call.
//         Gd::from_init_fn(|base| {
//             // Accept a base of type Base<Node3D> and directly forward it.
//             let mut message_area = Self {
//                 title,
//                 message,
//                 base,
//             };
//             {
//                 let mut base_mut = message_area.base_mut();
//                 base_mut.set_anchors_preset(LayoutPreset::FULL_RECT);
//                 // SAFETY: We add it to the scene tree right after.
//                 let mut wrapper = VBoxContainer::new_alloc();
//                 wrapper.set_anchor(Side::TOP, 0.4);
//                 wrapper.set_anchor(Side::BOTTOM, 0.6);
//                 wrapper.set_anchor(Side::LEFT, 0.4);
//                 wrapper.set_anchor(Side::RIGHT, 0.6);
//                 // Must be added otherwise we may have memory leaks.
//                 base_mut.add_child(wrapper);
//                 let mut panel = PanelContainer::new_alloc();
//             }
//             message_area
//         })
//     }
// }
// let mut scene_tree = self.base().get_tree().expect("able to get the scene tree");
// let mut os = Os::singleton();
//
// let mut root_node = scene_tree
//     .get_root()
//     .unwrap()
//     .get_node_as::<Node>("/root/Root");
//
// let mut title_screen = root_node
//     .find_child("TitleScreen".into())
//     .expect("custom menu button used outside of title screen");
