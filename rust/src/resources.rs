pub const TITLE_SCENE: &str = "res://scenes/menu/title.tscn";

pub const NORMAL_CHARACTER: &str = "res://scenes/character/character_3d.tscn";
pub const VR_CHARACTER: &str = "res://scenes/character/character_vr.tscn";

pub const WORLD_SCENE: &str = "res://scenes/game/world.tscn";
pub const QUESTION_ROOM_SCENE: &str = "res://scenes/game/room.tscn";

// Databank paths
pub const COURSE_DATA_FUNDAMENTALS: &str = "res://question_bank/data_fundamentals.json";
pub const COURSE_WEB_DEV: &str = "res://question_bank/web_dev.json";
pub const COURSE_AI_MODULES: &str = "res://question_bank/ai_modules.json";

// Material  paths
#[allow(unused)]
pub mod materials {
    pub const EMPTY: &str = "res://assets/empty_mat.tres";
    // Question Panel Materials
    pub const QP_BACKGROUND: &str = "res://assets/question_panel/background.tres";
    pub const QP_NORMAL: &str = "res://assets/question_panel/normal.tres";
    pub const QP_SUBMIT: &str = "res://assets/question_panel/submit.tres";
    pub const QP_SELECTED: &str = "res://assets/question_panel/selected.tres";
    pub const QP_HOVERED: &str = "res://assets/question_panel/hovered.tres";
    pub const QP_CHOICE_CORRECT: &str = "res://assets/question_panel/correct.tres";
    pub const QP_CHOICE_INCORRECT: &str = "res://assets/question_panel/incorrect.tres";
}
