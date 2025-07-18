pub mod core;
pub mod routine_state;
pub mod single_instance;
pub mod system_data;
mod system_tray;
mod ui_context;
pub mod view_state;

pub enum AppEvent {
    SystemTrayEvent(tray_icon::menu::MenuEvent),
}

#[derive(Debug, Clone)]
pub enum UIEvent {
    OpenWidgetScene(WidgetScene),

    RoutineChanged(RoutineChangeEvent, usize),
    UpdateViewState,
}

#[derive(Debug, Clone, Default)]
pub enum WidgetScene {
    #[default]
    Loading,
    RoutineList,
    RoutineDetail(usize),
    RoutineNew,
}

#[derive(Debug, Clone)]
pub enum RoutineChangeEvent {
    Rename(String),
    Run,
    Stop,
    Delete,
    SetRunWithStartup(bool),
}

impl RoutineChangeEvent {
    pub fn with_into(&self, index: usize) -> UIEvent {
        UIEvent::RoutineChanged(self.clone(), index)
    }
}

pub struct TerminateThreadEvent;
