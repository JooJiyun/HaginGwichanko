use crate::system::{
    {routine::RoutineInfo, view::RootViewElement},
    WidgetScene,
};

#[derive(Clone, Debug, Default)]
pub struct SystemData {
    pub count: usize,
    pub view_tree_processes: Vec<RootViewElement>,
    pub routines: Vec<RoutineInfo>,
    pub selected_routine_index: usize,
    pub current_widget_scene: WidgetScene,
}

impl SystemData {
    pub fn save(&self) {}

    pub fn load() -> Self {
        Self::default()
    }
}
