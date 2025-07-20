use crate::system::outview::RootViewElement;
use crate::system::routine::RoutineInfo;
use crate::system::WidgetScene;

#[derive(Clone, Debug, Default)]
pub struct AppData {
    pub current_widget_scene: WidgetScene,

    pub routines: Vec<RoutineInfo>,
    pub outview_trees: Vec<RootViewElement>,

    pub version_info: String,
}

impl AppData {
    pub fn save(&self) {}

    pub fn load() -> Self {
        Self::default()
    }
}
