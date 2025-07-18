use crate::system::{
    WidgetScene,
    {routine::RoutineInfo, view::RootViewElement},
};

#[derive(Clone, Debug, Default)]
pub struct SystemData {
    pub count: usize,

    pub current_widget_scene: WidgetScene,

    pub routines: Vec<RoutineInfo>,
    pub selected_routine_index: usize,
    pub is_selected_new: bool,

    pub view_tree_processes: Vec<RootViewElement>,
}

impl SystemData {
    pub fn save(&self) {}

    pub fn load() -> Self {
        Self::default()
    }
}
